import init, { evaluate_frg } from "./pkg/frg.js"

export async function evalFrg(sourceCode) {
	await init();
	
	const result = evaluate_frg(sourceCode);
	console.log(result);
}
