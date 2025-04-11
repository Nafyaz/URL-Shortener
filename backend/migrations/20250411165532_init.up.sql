-- Add up migration script here
CREATE TABLE urls (
                      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                      short_code VARCHAR(10) UNIQUE NOT NULL,
                      original_url varchar(2048) NOT NULL,
                      created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_short_code ON urls(short_code);

-- CREATE INDEX idx_original_url ON urls(original_url);