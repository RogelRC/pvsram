import { invoke } from "@tauri-apps/api/core";

export type ConceptType = "deposit" | "withdrawal" | "transfer";

export interface Concept {
    id: number;
    account_id: number;
    type: ConceptType;
    name: string;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export function listConcepts(params: {
    accountId: number;
    conceptType?: ConceptType | null;
    activeOnly?: boolean | null;
}): Promise<Concept[]> {
    return invoke("list_concepts", {
        accountId: params.accountId,
        conceptType: params.conceptType ?? null,
        activeOnly: params.activeOnly ?? true,
    });
}

export function createConcept(params: {
    accountId: number;
    conceptType: ConceptType;
    name: string;
}): Promise<Concept> {
    return invoke("create_concept", {
        accountId: params.accountId,
        conceptType: params.conceptType,
        name: params.name,
    });
}

/** Crea el concepto si no existe y lo devuelve. */
export function ensureConcept(params: {
    accountId: number;
    conceptType: ConceptType;
    name: string;
}): Promise<Concept> {
    return invoke("ensure_concept", {
        accountId: params.accountId,
        conceptType: params.conceptType,
        name: params.name,
    });
}
