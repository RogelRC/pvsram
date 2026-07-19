DROP TRIGGER IF EXISTS trg_delete_orphan_concepts;

CREATE TRIGGER trg_delete_orphan_concepts AFTER DELETE ON transaction_concepts FOR EACH ROW BEGIN
DELETE FROM concepts
WHERE
  NOT EXISTS (
    SELECT
      1
    FROM
      transaction_concepts tc
    WHERE
      tc.concept_id = concepts.id
  );

END;