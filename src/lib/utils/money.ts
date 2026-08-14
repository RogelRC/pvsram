/** Round a monetary value to 2 decimal places (cents). */
export function roundMoney(amount: number): number {
    return Math.round(amount * 100) / 100;
}

/** Format a monetary value for display with exactly 2 decimal places. */
export function formatMoney(amount: number): string {
    return roundMoney(amount).toLocaleString(undefined, {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    });
}
