// Shared DTE utility functions — used by home page and DTE detail page

export type DteEstado = 'pendiente' | 'manual' | 'fallido' | 'emitido';
export type PdfStatus = 'enviado' | 'pendiente';

interface DteBase {
	estado: DteEstado;
	esManual?: boolean;
}

/** Returns the CSS data-estado attribute value for badge styling. */
export function estadoKey(dte: DteBase): string {
	if (dte.estado === 'emitido' && dte.esManual) return 'emitido-manual';
	return dte.estado;
}

/** Returns the human-readable status label in Spanish. */
export function estadoLabel(dte: DteBase): string {
	if (dte.estado === 'emitido' && dte.esManual) return 'Emitido manual';
	const map: Record<DteEstado, string> = {
		pendiente: 'Pendiente',
		manual: 'Emisión manual',
		fallido: 'Fallido',
		emitido: 'Emitido',
	};
	return map[dte.estado];
}

/** Returns the neto (pre-tax) amount from a total with 19% IVA included. */
export function calcNeto(total: number): number {
	return Math.round(total / 1.19);
}

/** Returns the IVA (19%) amount from a total. */
export function calcIva(total: number): number {
	return total - calcNeto(total);
}
