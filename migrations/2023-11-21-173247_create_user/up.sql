CREATE TABLE "users" (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE NOT NULL,
    auth0_user_id TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);