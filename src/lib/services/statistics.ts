import { invoke } from "@tauri-apps/api/core";

export interface StatsFilters {
  from?: string | null;
  to?: string | null;
}

export interface AccountStats {
  account_id: number;
  account_name: string;
  account_number: string;
  currency: string;
  color: string | null;
  total_deposits: number;
  total_withdrawals: number;
  total_transfers_out: number;
  total_transfers_in: number;
  balance: number;
  transaction_count: number;
}

export interface CurrencyStats {
  currency: string;
  account_count: number;
  total_balance: number;
  total_deposits: number;
  total_withdrawals: number;
  total_transferred: number;
  transaction_count: number;
}

export function getStatsByAccount(
  filters: StatsFilters = {}
): Promise<AccountStats[]> {
  return invoke("get_stats_by_account", { filters });
}

export function getStatsByCurrency(
  filters: StatsFilters = {}
): Promise<CurrencyStats[]> {
  return invoke("get_stats_by_currency", { filters });
}
