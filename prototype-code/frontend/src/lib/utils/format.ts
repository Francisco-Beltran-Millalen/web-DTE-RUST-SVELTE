// Shared number formatting utilities

/** Formats a number as a Chilean peso amount with '$' prefix. e.g. $1.234.567 */
export function formatCLP(n: number): string {
	return '$' + n.toLocaleString('es-CL');
}

/** Formats a number with Chilean locale separators, no currency symbol. e.g. 1.234.567 */
export function formatNumber(n: number): string {
	return n.toLocaleString('es-CL');
}
