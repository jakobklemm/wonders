CREATE TYPE "fileCategory" AS ENUM (
  'audio',
  'image',
  'video',
  'docx',
  'uncategorized'
);

CREATE TABLE "challenge" (
  "id" guid PRIMARY KEY,
  "challengeId" int,
  "challengeTitle" varchar,
  "challengeDescription" text
);

CREATE TABLE "challengeEntry" (
  "id" guid PRIMARY KEY,
  "challenge" guid,
  "entreePersonName" varchar,
  "entryName" varchar
);

CREATE TABLE "challengeSubmission" (
  "id" guid PRIMARY KEY,
  "entry" guid,
  "fileName" varchar,
  "fileFileEnding" varchar,
  "fileCategory" fileCategory DEFAULT 'uncategorized',
  "fileCreatedOn" datetime
);

CREATE TABLE "challengeSubmissionMetadata" (
  "id" guid PRIMARY KEY,
  "submission" guid,
  "metadata" jsonb
);

ALTER TABLE "challengeEntry" ADD FOREIGN KEY ("challenge") REFERENCES "challenge" ("id");

ALTER TABLE "challengeSubmission" ADD FOREIGN KEY ("entry") REFERENCES "challengeEntry" ("id");

ALTER TABLE "challengeSubmissionMetadata" ADD FOREIGN KEY ("submission") REFERENCES "challengeSubmission" ("id");
