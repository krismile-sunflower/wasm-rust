import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { uuid } from './assets/pkg/hello_world'

function App() {
  const [count, setCount] = useState(0)

  const handleClick = async () => {
    // 直接引入了，刚才编译后的文件
    // const rust = import('./assets/pkg/hello_world.js');
    // rust
    //   .then(m => m.hello("Vite + React + Wasm"))
    //   .catch(console.error);

    // const result = await rust.then(m => m.add(2, 3));
    // console.log('2 + 3 =', result);
    // const id = await rust.then(m => m.uuid());
    // console.log('WASM生成的UUID:', id);

    const id = uuid();
    console.log('WASM生成的UUID:', id);

  }
  return (
    <>
      <div>
        <button onClick={handleClick}>点击我试一试</button>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
// wasm-bindgen target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir ./pkg

// wasm-bindgen target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir ../vite-project/src/assets/pkg --target web