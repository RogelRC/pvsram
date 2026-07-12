import { invoke } from "@tauri-apps/api/core";
import type { AccountStats } from "./statistics";

export interface MovementsFilters {
  from?: string | null; // "YYYY-MM-DD"
  to?: string | null; // "YYYY-MM-DD"
}

export function getMovementsReport(
  filters: MovementsFilters = {}
): Promise<AccountStats[]> {
  return invoke("get_movements_report", { filters });
}

export interface MonthlyBalanceFilters {
  year?: number | null;
  month?: number | null; // 1-12
}

export interface MonthlyBalanceReport {
  account_id: number;
  account_name: string;
  account_number: string;
  currency: string;
  color: string | null;
  opening_balance: number;
  closing_balance: number;
  net_change: number;
  period_start: string;
  period_end: string;
  month_finished: boolean;
}

export function getMonthlyBalanceReport(
  filters: MonthlyBalanceFilters = {}
): Promise<MonthlyBalanceReport[]> {
  return invoke("get_monthly_balance_report", { filters });
}
