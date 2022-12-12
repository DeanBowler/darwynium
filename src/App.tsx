import './App.css';

import { greet, add } from './native/pkg';

function App() {
  return (
    <div className="App">
      <h1>Darwynium</h1>
      <div className="card">
        <button onClick={() => greet(`visitor #${add(2, 4).toString()}`)}>
          hollar
        </button>
      </div>
    </div>
  );
}

export default App;
