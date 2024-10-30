## Database Setup

I am using timescale for ingestion but you can use anything there might be changes in the code accordingly

### Tables

- Users // Handling the user and api keys etc

  - id: uuid // This id works as api-key
  - username: String
  - createdAt: Date
  - updatedAt: Date

- ServerStats // Ingestion of server stats

  -

- OnchainStats // Ingestion of onchain stats
