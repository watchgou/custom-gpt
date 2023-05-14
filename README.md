

#### 使用rust开发chat-gpt接口
##### 第一阶段开发 Chat (including SSE streaming)和 Completions (including SSE streaming) 功能
```
chat
  curl \
	-u 'test:password01!' \
	-X POST '206.119.168.188:19002/chat' \
	-H 'Content-Type: application/json ' \
	-d '{ "msg":"你好" }'
```

```
completion
  curl \
	-u 'test:password01!' \
	-X POST '206.119.168.188:19002/completion' \
	-H 'Content-Type: application/json ' \
	-d '{ "msg":"你好","model":"text-davinci-003","max_token":7,"temperature":0}'	

```

##### 后续准备开发🔂 Images、Audio



