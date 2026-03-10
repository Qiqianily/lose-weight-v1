-- Add up migration script here
-- =====================================================
-- 1. 启用扩展
-- =====================================================
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =====================================================
-- 2. 创建枚举类型
-- =====================================================
-- CREATE TYPE user_identity AS ENUM ('user', 'vip', 'admin', 'unknown');
-- CREATE TYPE meal_type AS ENUM ('breakfast', 'lunch', 'dinner', 'extra_am', 'extra_pm');

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_identity') THEN
        CREATE TYPE user_identity AS ENUM ('user', 'vip', 'admin', 'unknown');
    END IF;
END $$;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'meal_type') THEN
        CREATE TYPE meal_type AS ENUM ('breakfast', 'lunch', 'dinner', 'extra_am', 'extra_pm');
    END IF;
END $$;

-- =====================================================
-- 3. 创建用户表 users
-- =====================================================
CREATE TABLE IF NOT EXISTS users (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,  -- 自增主键（推荐使用 IDENTITY）
    uuid UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,   -- 添加 UUID 字段，唯一且默认随机生成
    username VARCHAR(50) UNIQUE NOT NULL,                 -- 用户名，唯一且非空
    password VARCHAR(255) NOT NULL,                       -- 密码（存储哈希值）
    is_open BOOLEAN DEFAULT true,                         -- 是否启用，默认为 true
    identity user_identity NOT NULL DEFAULT 'user',       -- 身份枚举，默认为 user
    level INTEGER DEFAULT 0,                              -- 等级，默认 0
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,     -- 创建时间，默认当前时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,     -- 更新时间，默认当前时间
    last_login TIMESTAMPTZ                                 -- 最后登录时间，可为空
);

COMMENT ON TABLE users IS '用户基础信息表';
COMMENT ON COLUMN users.identity IS '用户身份（user/vip/admin/unknown）';

-- 身份枚举字段的索引（常用于筛选特定身份的用户）
CREATE INDEX IF NOT EXISTS idx_users_identity ON users(identity);

-- 等级字段的索引（常用于排序或范围查询）
CREATE INDEX IF NOT EXISTS idx_users_level ON users(level);

-- 最后登录时间的索引（常用于排序最近活跃用户）
CREATE INDEX IF NOT EXISTS idx_users_last_login ON users(last_login);

-- 可选：组合索引（若经常按身份和等级联合查询）
CREATE INDEX IF NOT EXISTS idx_users_identity_level ON users(identity, level);

-- =====================================================
-- 4. 创建体重数据表 data
-- =====================================================
CREATE TABLE IF NOT EXISTS data (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,      -- 自增主键
    uuid UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,      -- 公开唯一标识
    user_id INTEGER NOT NULL,                                  -- 关联用户 ID，非空
    weight NUMERIC(5,2) NOT NULL CHECK (weight > 0),                                      -- 体重 (kg)
    waistline NUMERIC(5,2) NOT NULL CHECK (waistline > 0),                                   -- 腰围 (cm)
    chest NUMERIC(5,2) CHECK (chest > 0),                                                -- 胸围 (cm)，可空
    arm NUMERIC(5,2) CHECK (arm > 0),                                                  -- 臂围 (cm)，可空
    leg NUMERIC(5,2) CHECK (leg > 0),                                                  -- 腿围 (cm)，可空
    fat NUMERIC(4,1) CHECK (fat >= 0 AND fat <= 100),                                                  -- 体脂率 (%)，可空
    date DATE NOT NULL DEFAULT CURRENT_DATE,                   -- 记录日期，默认当天
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 创建时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 更新时间

    -- 外键约束（关联 users 表）
    CONSTRAINT fk_data_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    -- 若需限制同一用户同一天只能有一条记录，可添加唯一约束：
    CONSTRAINT unique_user_data_date UNIQUE (user_id, date)
);

-- 添加注释
COMMENT ON TABLE data IS '用户每日体重数据记录';
COMMENT ON COLUMN data.weight IS '体重（kg）';
COMMENT ON COLUMN data.waistline IS '腰围（cm）';
COMMENT ON COLUMN data.chest IS '胸围（cm）';
COMMENT ON COLUMN data.arm IS '臂围（cm）';
COMMENT ON COLUMN data.leg IS '腿围（cm）';
COMMENT ON COLUMN data.fat IS '体脂率（%）';

-- 创建索引（提升查询性能）
-- 1 外键字段索引（加速关联查询）
CREATE INDEX idx_data_user_id ON data(user_id);

-- 2 日期字段索引（加速按日期范围查询）
CREATE INDEX idx_data_date ON data(date);

-- 3 组合索引（若常按用户和日期联合查询）
CREATE INDEX idx_data_user_date ON data(user_id, date);

-- =====================================================
-- 5. 创建体重管理目标表 goals
-- =====================================================
CREATE TABLE IF NOT EXISTS goals (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,       -- 自增主键
    uuid UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,       -- 公开唯一标识
    user_id INTEGER NOT NULL UNIQUE,                           -- 关联用户 ID，非空唯一
    height NUMERIC(5,2) NOT NULL CHECK (height > 0),                                      -- 身高 (cm)
    weight NUMERIC(5,2) NOT NULL CHECK (weight > 0),                                      -- 目标体重 (kg)
    waistline NUMERIC(5,2) NOT NULL CHECK (waistline > 0),                                   -- 目标腰围 (cm)
    chest NUMERIC(5,2) CHECK (chest > 0),                                                -- 目标胸围 (cm)，可空
    arm NUMERIC(5,2) CHECK (arm > 0),                                                  -- 目标臂围 (cm)，可空
    leg NUMERIC(5,2) CHECK (leg > 0),                                                  -- 目标腿围 (cm)，可空
    fat NUMERIC(4,1) CHECK (fat >= 0 AND fat <= 100),                                                  -- 目标体脂率 (%)，可空
    date DATE NOT NULL DEFAULT CURRENT_DATE,                   -- 目标日期（达成日期）
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 记录创建时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 记录最后更新时间
    bmi NUMERIC(4,1) GENERATED ALWAYS AS (weight / ((height / 100)^2)) STORED,                                         -- 体重指数 (BMI)
    -- 外键约束（关联 users 表，用户删除时相应目标级联删除）
    CONSTRAINT fk_goals_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 添加注释
COMMENT ON TABLE goals IS '用户身体目标数据（每人一条）';
COMMENT ON COLUMN goals.height IS '身高（cm）';
COMMENT ON COLUMN goals.bmi IS '体重指数（自动计算）';
COMMENT ON COLUMN goals.date IS '目标达成日期';

-- 创建索引提升查询性能
CREATE INDEX IF NOT EXISTS idx_goals_user_id ON goals(user_id);        -- 外键索引
CREATE INDEX IF NOT EXISTS idx_goals_date ON goals(date);              -- 日期索引

-- =====================================================
-- 6. 创建三餐记录表 diets
-- =====================================================
CREATE TABLE IF NOT EXISTS diets (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,       -- 自增主键
    uuid UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,       -- 公开唯一标识
    user_id INTEGER NOT NULL,                                  -- 关联用户 ID，非空
    breakfast TEXT NOT NULL,                                   -- 早餐
    extra_am TEXT,                                             -- 上午加餐
    lunch TEXT NOT NULL,                                       -- 午餐
    extra_pm TEXT,                                             -- 下午加餐
    dinner TEXT NOT NULL,                                      -- 晚餐
    date DATE NOT NULL DEFAULT CURRENT_DATE,                   -- 日期
    comment TEXT,                                              -- 备注
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,          -- 记录创建时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,          -- 记录最后更新时间
    -- 外键约束（关联 users 表，用户删除时相应目标级联删除）
    CONSTRAINT fk_diets_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    -- 若需限制同一用户同一天只能有一条记录，可添加唯一约束：
    CONSTRAINT unique_user_diets_date UNIQUE (user_id, date)
);

COMMENT ON TABLE diets IS '用户每日饮食记录';
COMMENT ON COLUMN diets.breakfast IS '早餐内容';
COMMENT ON COLUMN diets.extra_am IS '上午加餐';
COMMENT ON COLUMN diets.lunch IS '午餐内容';
COMMENT ON COLUMN diets.extra_pm IS '下午加餐';
COMMENT ON COLUMN diets.dinner IS '晚餐内容';

-- 索引
CREATE INDEX IF NOT EXISTS idx_diets_user_id ON diets(user_id);
CREATE INDEX IF NOT EXISTS idx_diets_date ON diets(date);
CREATE INDEX IF NOT EXISTS idx_diets_user_date ON diets(user_id, date);

-- =====================================================
-- 7. 创建饮食图片表 diet_images
-- =====================================================
CREATE TABLE IF NOT EXISTS diet_images (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,
    diet_id INTEGER NOT NULL,                                 -- 关联到 diets 表
    meal_type meal_type NOT NULL,                             -- 标识：早餐/午餐/晚餐/上午加餐/下午加餐
    image_url TEXT NOT NULL,                                  -- 图片访问地址
    sort_order SMALLINT NOT NULL DEFAULT 0,                   -- 同一餐内的图片排序
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 记录创建时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,         -- 记录最后更新时间
    CONSTRAINT fk_diet_images_diet FOREIGN KEY (diet_id) REFERENCES diets(id) ON DELETE CASCADE
);

COMMENT ON TABLE diet_images IS '饮食记录关联图片';
COMMENT ON COLUMN diet_images.meal_type IS '餐别（早/午/晚/加餐）';
COMMENT ON COLUMN diet_images.sort_order IS '同一餐内的图片排序序号';

-- 索引加速查询（按饮食记录 + 餐别过滤）
CREATE INDEX idx_diet_images_diet_meal ON diet_images(diet_id, meal_type);

-- 如果需要按餐别和排序顺序展示，可创建组合索引
CREATE INDEX idx_diet_images_diet_meal_sort ON diet_images(diet_id, meal_type, sort_order);

-- =====================================================
-- 8. 创建自动更新 updated_at 的触发器函数
-- =====================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为所有需要自动更新 updated_at 的表添加触发器
DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT table_name FROM information_schema.tables
        WHERE table_schema = 'public'
          AND table_name IN ('users', 'data', 'goals', 'diets', 'diet_images')
    LOOP
        EXECUTE format('
            DROP TRIGGER IF EXISTS trigger_update_%I ON %I;
            CREATE TRIGGER trigger_update_%I
            BEFORE UPDATE ON %I
            FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
        ', t, t, t, t);
    END LOOP;
END;
$$ LANGUAGE plpgsql;
