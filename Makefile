BINARY = ./target/debug/discord_command # バイナリファイル
OPTION = -t=20 # 正常オプション
COMMAND := ${BINARY} ${OPTION} # 正常コマンド

# デバッグビルド
build:
	cargo build

# 実行
run:
	${COMMAND}

# ビルド＆実行
exec:
	make build
	make run
