# 词典数据库说明

## 概述
`dict.db` 是一个 SQLite 数据库文件，用于存储词典数据。该数据库包含单词的详细信息，包括释义、音标、词性、词频等。

## 数据库结构

### 表名：`dict`

| 字段名 | 数据类型 | 说明 |
|--------|----------|------|
| `word` | TEXT | **主键**，单词本身 |
| `phonetic` | TEXT | 音标 |
| `definition` | TEXT | 英文释义 |
| `translation` | TEXT | 中文翻译 |
| `pos` | TEXT | 词性（Part of Speech） |
| `collins` | INTEGER | 柯林斯星级（1-5星） |
| `oxford` | INTEGER | 是否牛津收录（0=否，1=是） |
| `tag` | TEXT | 标签（如 CET4、CET6、GRE、TOEFL 等） |
| `bnc` | INTEGER | BNC（英国国家语料库）词频 |
| `frq` | INTEGER | COCA（美国当代英语语料库）词频 |
| `exchange` | TEXT | 词形变化（JSON 格式或字符串） |
| `detail` | TEXT | 详细解释（JSON 格式或长文本） |
| `audio` | TEXT | 音频文件地址 |

## SQL 创建语句

```sql
CREATE TABLE dict (
    word TEXT NOT NULL PRIMARY KEY,                 -- 单词
    phonetic TEXT,                      -- 音标
    definition TEXT,                    -- 英文释义
    translation TEXT,                   -- 中文翻译
    pos TEXT,                           -- 词性
    collins INTEGER,                    -- 柯林斯星级
    oxford INTEGER,                     -- 是否牛津收录（0/1）
    tag TEXT,                           -- 标签（如 CET4 / GRE）
    bnc INTEGER,                        -- BNC 词频
    frq INTEGER,                        -- COCA 词频
    exchange TEXT,                      -- 词形变化（JSON或字符串）
    detail TEXT,                        -- 详细解释（JSON或长文本）
    audio TEXT                          -- 音频地址
);
```

## 使用示例

### 查询单词信息
```sql
SELECT * FROM dict WHERE word = 'example';
```

### 查询特定词性的单词
```sql
SELECT word, translation, pos FROM dict WHERE pos LIKE '%verb%';
```

### 查询高频词汇（按 COCA 词频排序）
```sql
SELECT word, translation, frq FROM dict WHERE frq > 1000 ORDER BY frq DESC;
```

### 查询柯林斯五星词汇
```sql
SELECT word, translation, collins FROM dict WHERE collins = 5;
```

## 数据来源说明

该词典数据可能来源于以下一个或多个来源：
- 柯林斯词典
- 牛津词典
- 其他公开词典资源

## 注意事项

1. **数据完整性**：数据库中的某些字段可能为空，取决于数据源。
2. **音频文件**：`audio` 字段存储的是音频文件的相对路径或 URL，需要确保音频文件存在或可访问。
3. **JSON 字段**：`exchange` 和 `detail` 字段可能包含 JSON 格式的数据，解析时需要注意。

## 维护与更新

如需更新数据库结构，请同步更新：
1. 本 README 文件
2. `docs/sql/dict.sql` 文件中的 SQL 定义
3. 应用程序中相关的数据模型定义

---

*最后更新：2026-03-17*