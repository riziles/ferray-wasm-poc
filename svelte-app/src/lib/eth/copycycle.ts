// The one step of the paper that fits on a page, and the only one whose claim
// is exactly checkable: the counting-preserving reduction from an arbitrary
// 3-CNF to one where every variable occurs at most three times (Section 2.8,
// following Tovey's implication cycle).
//
// The transformation, verbatim from the paper's recipe:
//
//   1. Clauses of size ≤ 3 are kept; longer ones are rewritten through OR gates
//      with the Tseitin clauses (not needed here — we generate 3-CNFs directly).
//   2. If a variable x occurs s ⩾ 2 times, its occurrences are replaced by
//      distinct copies x_1, …, x_s (signs preserved) and the implication cycle
//      (¬x_1 ∨ x_2) ∧ … ∧ (¬x_{s-1} ∨ x_s) ∧ (¬x_s ∨ x_1) is added. The cycle
//      forces all copies to agree, so satisfying assignments correspond
//      bijectively.
//   3. Variables with at most one occurrence are kept unchanged.
//
// Each copy then appears in one inherited clause and two cycle clauses, hence in
// exactly three. The count is preserved — which is the whole point, and which
// this module lets the page verify by counting both sides.

export interface Lit {
	/** 1-based variable index. */
	v: number;
	neg: boolean;
}

export interface Formula {
	n: number;
	clauses: Lit[][];
}

/** Deterministic PRNG so every reader of the page sees the same formulas. */
export function mulberry32(seed: number) {
	return function next() {
		seed |= 0;
		seed = (seed + 0x6d2b79f5) | 0;
		let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

/** A random 3-CNF: m clauses, three distinct variables each, random signs. */
export function randomFormula(n: number, m: number, seed: number): Formula {
	if (n < 3) throw new Error('randomFormula needs at least 3 variables');
	const rnd = mulberry32(seed);
	const clauses: Lit[][] = [];
	for (let e = 0; e < m; e++) {
		const picked = new Set<number>();
		while (picked.size < 3) picked.add(1 + Math.floor(rnd() * n));
		clauses.push([...picked].map((v) => ({ v, neg: rnd() < 0.5 })));
	}
	return { n, clauses };
}

export interface Rewrite {
	formula: Formula;
	/** copies[v - 1] = how many copies variable v was split into (1 = unchanged). */
	copies: number[];
	/** Occurrences per variable before and after. */
	occBefore: number[];
	occAfter: number[];
}

/**
 * The variable-copy reduction with Tovey's implication cycle. Occurrence counts
 * are capped at 3 by construction: each copy inherits one clause and joins two
 * cycle clauses.
 */
export function copyAndCycle(f: Formula): Rewrite {
	const occBefore = new Array(f.n).fill(0);
	for (const clause of f.clauses) for (const lit of clause) occBefore[lit.v - 1] += 1;

	// Variables occurring ⩽ 1 time keep their index; the rest get one per occurrence.
	const copies = occBefore.map((c) => (c >= 2 ? c : 1));
	const base: number[] = [];
	let next = 0;
	for (let v = 0; v < f.n; v++) {
		base.push(next);
		next += copies[v];
	}

	// Rename occurrences positionally: the k-th occurrence of v becomes copy k.
	const used = new Array(f.n).fill(0);
	const clauses: Lit[][] = f.clauses.map((clause) =>
		clause.map((lit) => {
			const s = occBefore[lit.v - 1];
			if (s < 2) return { v: base[lit.v - 1] + 1, neg: lit.neg };
			const k = used[lit.v - 1]++;
			return { v: base[lit.v - 1] + 1 + k, neg: lit.neg };
		})
	);

	// Implication cycles, one per split variable: (¬x_k ∨ x_{k+1}) around the ring.
	for (let v = 0; v < f.n; v++) {
		const s = copies[v];
		if (s < 2) continue;
		for (let k = 0; k < s; k++) {
			const a = base[v] + 1 + k;
			const b = base[v] + 1 + ((k + 1) % s);
			clauses.push([
				{ v: a, neg: true },
				{ v: b, neg: false }
			]);
		}
	}

	const formula: Formula = { n: next, clauses };
	const occAfter = new Array(next).fill(0);
	for (const clause of formula.clauses) for (const lit of clause) occAfter[lit.v - 1] += 1;

	return { formula, copies, occBefore, occAfter };
}

/**
 * Exact model count: branch on the busiest free variable, with unit propagation
 * and conflict detection. Exact (unlike a sampling heuristic) and fast enough for
 * the ~30-variable rewritten formula, where a 2^n enumeration would not be.
 *
 * Each call works on its own copy of the partial assignment, because propagation
 * assigns variables as a side effect.
 */
export function countModels(f: Formula): number {
	const assign: (0 | 1 | null)[] = new Array(f.n).fill(null);
	return count(f.clauses, assign, f.n);
}

function count(clauses: Lit[][], inherited: (0 | 1 | null)[], n: number): number {
	const assign = inherited.slice();

	// Propagate units to a fixed point; a clause with no free literal is a conflict.
	for (;;) {
		let progressed = false;
		for (const clause of clauses) {
			let freeCount = 0;
			let freeLit: Lit | null = null;
			let satisfied = false;
			for (const lit of clause) {
				const a = assign[lit.v - 1];
				if (a === null) {
					freeCount += 1;
					freeLit = lit;
				} else if ((a === 1) !== lit.neg) {
					satisfied = true;
					break;
				}
			}
			if (satisfied) continue;
			if (freeCount === 0) return 0;
			if (freeCount === 1 && freeLit) {
				assign[freeLit.v - 1] = freeLit.neg ? 0 : 1;
				progressed = true;
			}
		}
		if (!progressed) break;
	}

	// Any clause still open? Count free variables, and weight them by occurrence.
	const weight = new Array(n).fill(0);
	let free = 0;
	let open = 0;
	for (const clause of clauses) {
		let satisfied = false;
		for (const lit of clause) {
			const a = assign[lit.v - 1];
			if (a !== null && (a === 1) !== lit.neg) satisfied = true;
		}
		if (satisfied) continue;
		open += 1;
		for (const lit of clause) if (assign[lit.v - 1] === null) weight[lit.v - 1] += 1;
	}
	for (let i = 0; i < n; i++) if (assign[i] === null) free += 1;
	// Every clause satisfied: all still-free variables are unconstrained.
	if (open === 0) return Math.pow(2, free);

	let best = -1;
	for (let i = 0; i < n; i++) {
		if (assign[i] === null && (best < 0 || weight[i] > weight[best])) best = i;
	}
	if (best < 0) return 0;

	let total = 0;
	for (const value of [0, 1] as const) {
		assign[best] = value;
		total += count(clauses, assign, n);
	}
	return total;
}

/** How many times each variable occurs, as a small histogram {occurrences: count}. */
export function histogram(occ: number[]): { k: number; vars: number }[] {
	const m = new Map<number, number>();
	for (const o of occ) m.set(o, (m.get(o) ?? 0) + 1);
	return [...m.entries()].sort((a, b) => a[0] - b[0]).map(([k, vars]) => ({ k, vars }));
}

/** 1,2,3 -> "x₁ ∨ ¬x₂ ∨ x₃" */
const SUB = '₀₁₂₃₄₅₆₇₈₉';
const sub = (i: number) =>
	String(i)
		.split('')
		.map((d) => SUB[Number(d)])
		.join('');

export function litText(l: Lit): string {
	return `${l.neg ? '¬' : ''}x${sub(l.v)}`;
}

export function clauseText(c: Lit[]): string {
	return c.map(litText).join(' ∨ ');
}
