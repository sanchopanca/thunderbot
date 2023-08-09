CREATE TABLE rules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE patterns (
    id INTEGER PRIMARY KEY,
    pattern TEXT NOT NULL,
    rule_id INTEGER NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX pattern_rule_fk_idx on patterns(rule_id);

CREATE TABLE responses (
    id INTEGER PRIMARY KEY,
    response TEXT NOT NULL,
    rule_id INTEGER NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX response_rule_fk_idx on responses(rule_id);


CREATE TABLE tokens (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL,
    user TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX token_idx on tokens(token);
