CREATE TABLE IF NOT EXISTS task (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	title text NOT NULL,
	description text
);