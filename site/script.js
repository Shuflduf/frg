async function init() {
  const importObj = {};

  const response = await fetch("./target/wasm32-unknown-unknown/release/frg.wasm");
  const {instance} =
  await WebAssembly.instantiateStreaming(response, importObj);
  console.log(instance.exports.evaluate_frg);
  console.log(JSON.stringify(instance.exports));
}

// init()

