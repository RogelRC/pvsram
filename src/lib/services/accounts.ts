import { invoke } from "@tauri-apps/api/core";

export interface Account {
  id: number;
  number: string;
  name: string;
  description: string | null;
  owner: string;
  currency: string;
  color: string | null;
  created_at: string;
  updated_at: string;
}

export function createAccount(params: {
  number: string;
  name: string;
  description?: string | null;
  owner: string;
  currency?: string | null;
  color?: string | null;
}): Promise<Account> {
  return invoke("create_account", params);
}

export function listAccounts(): Promise<Account[]> {
  return invoke("list_accounts");
}

export function getAccount(id: number): Promise<Account> {
  return invoke("get_account", { id });
}

export function updateAccount(params: {
  id: number;
  number: string;
  name: string;
  description?: string | null;
  owner: string;
  currency: string;
  color?: string | null;
}): Promise<Account> {
  return invoke("update_account", params);
}

export function deleteAccount(id: number): Promise<void> {
  return invoke("delete_account", { id });
}
