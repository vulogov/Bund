curl http://localhost:11434/api/chat -d '{
  "model": "llama2",
  "stream": false,
  "messages": [
    { "role": "user", "content": "what is the answer on life universe and everything?" }
  ]
}'
