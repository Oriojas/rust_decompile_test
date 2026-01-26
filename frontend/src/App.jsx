import React, { useState } from 'react';
import TransactionForm from './components/TransactionForm';
import RiskAnalysis from './components/RiskAnalysis';
import { analyzeRisk } from './api';

function App() {
  const [analysisData, setAnalysisData] = useState(null);
  const [loading, setLoading] = useState(false);

  const handleAnalyze = async (address, callData) => {
    setLoading(true);
    setAnalysisData(null);
    try {
      const result = await analyzeRisk(address, callData);
      setAnalysisData(result);
    } catch (error) {
      console.error(error);
      setAnalysisData({ status: 'error', message: 'Unknown error occurred' });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app-container">
      <header style={{ textAlign: 'center', marginBottom: '3rem', paddingTop: '2rem' }}>
        <h1 className="cyber-title" style={{ fontSize: '3rem', margin: 0 }}>
          RISK<span style={{ color: '#fff' }}>SCANNER</span>
        </h1>
        <p className="cyber-text" style={{ color: 'var(--neon-cyan)', marginTop: '0.5rem' }}>
          Arbitrum Sepolia Transaction Analyzer
        </p>
      </header>

      <main>
        <TransactionForm onSubmit={handleAnalyze} loading={loading} />
        {analysisData && <RiskAnalysis data={analysisData} />}
      </main>

      <footer style={{ marginTop: '4rem', textAlign: 'center', opacity: 0.5, fontSize: '0.8rem' }}>
        <p className="cyber-text">SECURE // DECENTRALIZE // VERIFY</p>
      </footer>
    </div>
  );
}

export default App;
