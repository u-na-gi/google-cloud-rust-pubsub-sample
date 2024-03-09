import json
import sys

# 書き捨てスクリプト

# JSONファイルのパスを指定
# コマンドライン引数からファイルパスを取得
if len(sys.argv) != 2:
    print("Usage: python script.py full_your_file_path.json")
    sys.exit(1)

file_path = sys.argv[1]

# JSONファイルを読み込み、1行の文字列として出力
with open(file_path, 'r') as json_file:
    data = json.load(json_file)
    one_line_str = json.dumps(data)
    escaped_str = json.dumps(one_line_str)
    
# .envファイルに追記
with open('./secret/.env', 'a') as env_file:
    # 環境変数のキーをYOUR_ENV_KEYに設定（適宜変更してください）
    env_file.write(f"\nGCP_SERVICE_ACCOUNT_CREDENTIAL={escaped_str}")

