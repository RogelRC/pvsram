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
    asOf?: string | null,
): Promise<number> {
    return invoke("get_account_balance", { accountId, asOf: asOf ?? null });
}

// ============ Reportes / Historial con filtros flexibles ============

export interface TransactionFilters {
    accountId?: number | null;
    relatedAccountId?: number | null;
    type?: "deposit" | "withdrawal" | "transfer" | null;
    currency?: string | null;
    amountMin?: number | null;
    amountMax?: number | null;
    description?: string | null;
    from?: string | null;
    to?: string | null;
    sortBy?: "occurred_at" | "amount" | null;
    sortDir?: "asc" | "desc" | null;
    limit?: number | null;
    offset?: number | null;
}

export interface TransactionRecord extends Transaction {
    account_name: string;
    account_number: string;
    currency: string;
    related_account_name: string | null;
    related_account_number: string | null;
}

export interface TransactionStats {
    total_count: number;
    total_deposits: number;
    total_withdrawals: number;
    total_transfers: number;
}

export function listTransactionsReport(
    filters: TransactionFilters = {},
): Promise<TransactionRecord[]> {
    return invoke("list_transactions_report", { filters });
}

export function countTransactionsReport(
    filters: TransactionFilters = {},
): Promise<number> {
    return invoke("count_transactions_report", { filters });
}

export function getTransactionsStats(
    filters: TransactionFilters = {},
): Promise<TransactionStats> {
    return invoke("get_transactions_stats", { filters });
}

export function updateTransaction(params: {
    id: number;
    relatedAccountId?: number | null;
    amount: number;
    description?: string | null;
    occurredAt: string;
}): Promise<Transaction> {
    return invoke("update_transaction", {
        id: params.id,
        relatedAccountId: params.relatedAccountId ?? null,
        amount: params.amount,
        description: params.description ?? null,
        occurredAt: params.occurredAt,
    });
}

export function deleteTransaction(id: number): Promise<void> {
    return invoke("delete_transaction", { id });
}
