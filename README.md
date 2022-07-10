# note-search
search my own notes

## 设计
  - 命令设计
    - 添加目录或文件到待提交列表(可以对此次添加的文件附带一个备注信息，方便搜索) ns add $path [-m 'note message'] 
    - 展示所有待提交列表 ns status
    - 提交给搜索引擎 ns commit 
    - 显示当前索引或添加并切换到指定索引 ns index [index name]
    - 显示所有索引 ns indexes
    - 删除索引 ns index delete [index name]
    - 从搜索引擎中删除指定文档 ns rm [doc id]
  - 配置文件设计
    ```yaml
    server:
      host: 127.0.0.1
      port: 7700
    processor: #default reader BinaryReader(read nothing,just return an empty string)
      - extensions: [".md",".txt"]
        reader: PlainTextReader
      - extensions: [".doc",".docx"]
        reader: WordReader
    ```
  - 数据格式设计
    - id 路径哈希值
    - path 文件绝对路径
    - name 文件名
    - content 文件内容
    - updated 文件最后修改时间（从文件本身属性读取）
    - remark/note 文件备注
