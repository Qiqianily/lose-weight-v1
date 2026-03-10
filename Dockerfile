# 使用 alpine 作为基础镜像，打包进可执行文件，体积更小，启动速度更快，10MB 左右
FROM alpine:latest

# 创建工作目录 /app 和日志目录 /app/logs
RUN mkdir -p /app/data /app/conf /app/logs

# 复制编译好的 Rust 应用程序到容器中
COPY lose_weight /app/lose_weight

# 复制配置文件到容器中，在运行时有数据卷挂载。
#COPY prod.yml /app/conf/prod.yml

# 设置工作目录
WORKDIR /app

# 暴露应用程序使用的端口（根据实际情况修改）
EXPOSE 8989

# 启动命令
CMD ["./lose_weight","--config", "/app/conf/prod.yml"]
