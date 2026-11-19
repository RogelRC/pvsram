/** Round a monetary value to 2 decimal places (cents). */
export function roundMoney(amount: number): number {
    return Math.round((amount + Number.EPSILON) * 100) / 100;
}

/**
 * Format a number with apostrophe (') as thousands separator.
 * Example: 1000.5 -> "1'000.50" (decimals=2)
 */
export function formatNumber(
    value: number,
    decimals = 2,
    options?: { minDecimals?: number },
): string {
    if (!Number.isFinite(value)) return String(value);

    const fixed = Math.abs(value).toFixed(decimals);
    const [intPartRaw, fracPart = ""] = fixed.split(".");
    const intPart = intPartRaw.replace(/\B(?=(\d{3})+(?!\d))/g, "'");
    const sign = value < 0 && Number(fixed) !== 0 ? "-" : "";

    let frac = fracPart;
    if (options?.minDecimals !== undefined && options.minDecimals < decimals) {
        // Trim trailing zeros down to minDecimals
        while (frac.length > options.minDecimals && frac.endsWith("0")) {
            frac = frac.slice(0, -1);
        }
    }

    return frac.length > 0 ? `${sign}${intPart}.${frac}` : `${sign}${intPart}`;
}

/** Format a monetary value for display with exactly 2 decimal places. */
export function formatMoney(amount: number): string {
    return formatNumber(roundMoney(amount), 2);
}

/** Format an integer-like count with apostrophe thousands separator. */
export function formatCount(value: number): string {
    if (!Number.isFinite(value)) return String(value);
    return formatNumber(Math.trunc(value), 0);
}
