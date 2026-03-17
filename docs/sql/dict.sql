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