BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "user" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "avatar_url" TEXT
);
CREATE TABLE IF NOT EXISTS "repo" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    "description" TEXT,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    FOREIGN KEY("owner") REFERENCES "user"("id")
);
CREATE TABLE IF NOT EXISTS "post" (
    "id" TEXT PRIMARY KEY,
    "category" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "author" TEXT NOT NULL,
    "repo_id" TEXT NOT NULL,
    FOREIGN KEY("author") REFERENCES "user"("id"),
    FOREIGN KEY("repo_id") REFERENCES "repo"("id")
);
CREATE TABLE IF NOT EXISTS "subscribe" (
    "id" TEXT PRIMARY KEY,
    "user_id" TEXT NOT NULL,
    "repo_id" TEXT NOT NULL,
    FOREIGN KEY("user_id") REFERENCES "user"("id"),
    FOREIGN KEY("repo_id") REFERENCES "repo"("id")
);
CREATE TABLE IF NOT EXISTS "comment" (
    "id" TEXT PRIMARY KEY,
    "post_id" TEXT NOT NULL,
    "repo_id" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "author" TEXT NOT NULL,
    "parent_id" TEXT,
    FOREIGN KEY("author") REFERENCES "user"("id"),
    FOREIGN KEY("post_id") REFERENCES "post"("id"),
    FOREIGN KEY("repo_id") REFERENCES "repo"("id")
);
COMMIT;
