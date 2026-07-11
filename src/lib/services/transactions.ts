import { invoke } from "@tauri-apps/api/core";

export interface Transaction {
  id: number;
  type: "deposit" | "withdrawal" | "transfer";
  account_id: number;
  related_account_id: number | null;
  amount: number;
  description: string | null;
  occurred_at: string;
  created_at: string;
  updated_at: string;
}

export function createDeposit(params: {
  accountId: number;
  amount: number;
  description?: string | null;
  occurredAt?: string | null;
}): Promise<Transaction> {
  return invoke("create_deposit", {
    accountId: params.accountId,
    amount: params.amount,
    description: params.description ?? null,
    occurredAt: params.occurredAt ?? null,
  });
}

export function createWithdrawal(params: {
  accountId: number;
  amount: number;
  description?: string | null;
  occurredAt?: string | null;
}): Promise<Transaction> {
  return invoke("create_withdrawal", {
    accountId: params.accountId,
    amount: params.amount,
    description: params.description ?? null,
    occurredAt: params.occurredAt ?? null,
  });
}

export function createTransfer(params: {
  accountId: number;
  relatedAccountId: number;
  amount: number;
  description?: string | null;
  occurredAt?: string | null;
}): Promise<Transaction> {
  return invoke("create_transfer", {
    accountId: params.accountId,
    relatedAccountId: params.relatedAccountId,
    amount: params.amount,
    description: params.description ?? null,
    occurredAt: params.occurredAt ?? null,
  });
}

export function listTransactions(params?: {
  accountId?: number | null;
  from?: string | null;
  to?: string | null;
}): Promise<Transaction[]> {
  return invoke("list_transactions", {
    accountId: params?.accountId ?? null,
    from: params?.from ?? null,
    to: params?.to ?? null,
  });
}

export function getAccountBalance(
  accountId: number,
  asOf?: string | null
): Promise<number> {
  return invoke("get_account_balance", { accountId, asOf: asOf ?? null });
}
