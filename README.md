

#### 使用rust开发chat-gpt接口
##### 第一阶段开发 Chat (including SSE streaming)和 Completions (including SSE streaming) 功能
<a href="http://170.106.154.186:19002/gpt/index.html" title="">网页</a>
#### chat
```
  curl \
	-X POST '170.106.154.186:19002/chat' \
	-H 'Content-Type: application/json ' \
	-H 'authorization:eyJhbGciOiJIUzI1NiJ9.eyJyb2xlIjoiQURNSU46OlVTRVIiLCJ0b2tlbklkIjoiZDhiMDkxYTlmYmQ0NDcyZThiYmZkMWIzNWIzNThlMjEiLCJ1c2VyTmFtZSI6ImFkbWluIiwiZXhwIjoxNjg0NjA1OTk2fQ.91douCfJo0Jyj9js2KWlyLvqrHLLhRiUw6ltNMwOVY8' \
	-d '{ "msg":"你好" }'
```

#### completion
```
  curl \
	-X POST '170.106.154.186:19002/completion' \
	-H 'Content-Type: application/json ' \
	-H 'authorization:eyJhbGciOiJIUzI1NiJ9.eyJyb2xlIjoiQURNSU46OlVTRVIiLCJ0b2tlbklkIjoiZDhiMDkxYTlmYmQ0NDcyZThiYmZkMWIzNWIzNThlMjEiLCJ1c2VyTmFtZSI6ImFkbWluIiwiZXhwIjoxNjg0NjA1OTk2fQ.91douCfJo0Jyj9js2KWlyLvqrHLLhRiUw6ltNMwOVY8' \
	-d '{ "msg":"就算全世界离开你"}'	

```

##### 后续准备开发🔂 Images、Audio



