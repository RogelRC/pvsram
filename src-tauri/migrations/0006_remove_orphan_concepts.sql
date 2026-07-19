CREATE TRIGGER IF NOT EXISTS trg_delete_orphan_concepts AFTER DELETE ON transaction_concepts FOR EACH ROW BEGIN
DELETE FROM concepts
WHERE
  id = OLD.concept_id
  AND NOT EXISTS (
    SELECT
      1
    FROM
      transaction_concepts
    WHERE
      concept_id = OLD.concept_id
  );

END;