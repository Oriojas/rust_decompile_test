import React from 'react';
import ReactMarkdown from 'react-markdown';

export default function RiskAnalysis({ data }) {
    if (!data) return null;

    const { status, risk_level, explanation, function_name, arguments: args, message } = data;
    const isError = status === 'error';

    return (
        <div className="cyber-panel" style={{ marginTop: '2rem', borderColor: isError ? 'var(--neon-red)' : 'var(--neon-cyan)' }}>
            <h2 className="cyber-text" style={{ color: isError ? 'var(--neon-red)' : 'var(--neon-green)', marginBottom: '1.5rem' }}>
                {isError ? '! SYSTEM ERROR !' : '> ANALYSIS COMPLETE'}
            </h2>

            {risk_level && (
                <div style={{ marginBottom: '1.5rem', textAlign: 'center' }}>
                    <span className="cyber-text" style={{ fontSize: '0.9rem', marginRight: '1rem' }}>RISK LEVEL:</span>
                    <span className={`risk-badge ${risk_level.toLowerCase().includes('alto') || risk_level.toLowerCase().includes('high') ? 'risk-high' :
                        risk_level.toLowerCase().includes('medio') || risk_level.toLowerCase().includes('medium') ? 'risk-medium' :
                            'risk-low'
                        }`}>
                        {risk_level.toUpperCase()}
                    </span>
                </div>
            )}

            <div style={{ background: 'rgba(0,0,0,0.3)', padding: '1rem', borderRadius: '4px', marginBottom: '1rem' }}>
                <h3 className="cyber-text" style={{ fontSize: '0.9rem', color: '#888', marginBottom: '0.5rem' }}>
          // DETECTED FUNCTION
                </h3>
                <p style={{ color: 'var(--neon-pink)', fontFamily: 'var(--font-mono)' }}>
                    {function_name ? `${function_name}(...)` : 'Unknown'}
                </p>
            </div>

            {args && args.length > 0 && (
                <div style={{ background: 'rgba(0,0,0,0.3)', padding: '1rem', borderRadius: '4px', marginBottom: '1rem' }}>
                    <h3 className="cyber-text" style={{ fontSize: '0.9rem', color: '#888', marginBottom: '0.5rem' }}>
            // DECODED PARAMS
                    </h3>
                    <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                        {args.map((arg, idx) => (
                            <li key={idx} style={{ marginBottom: '0.5rem', wordBreak: 'break-all' }}>
                                <span style={{ color: 'var(--neon-cyan)' }}>[{idx}]</span> {arg}
                            </li>
                        ))}
                    </ul>
                </div>
            )}

            <div style={{ background: 'rgba(0,0,0,0.3)', padding: '1rem', borderRadius: '4px' }}>
                <h3 className="cyber-text" style={{ fontSize: '0.9rem', color: '#888', marginBottom: '0.5rem' }}>
          // SYSTEM ANALYSIS
                </h3>
                <div style={{ lineHeight: '1.6', color: '#ccc', fontSize: '0.95rem' }} className="markdown-content">
                    {explanation ? (
                        <ReactMarkdown
                            components={{
                                strong: ({ node, ...props }) => <strong style={{ color: 'var(--neon-cyan)', fontWeight: 'bold' }} {...props} />,
                                ul: ({ node, ...props }) => <ul style={{ paddingLeft: '1.5rem' }} {...props} />,
                                li: ({ node, ...props }) => <li style={{ marginBottom: '0.5rem' }} {...props} />
                            }}
                        >
                            {explanation}
                        </ReactMarkdown>
                    ) : (
                        <p>{message || "No details available."}</p>
                    )}
                </div>
            </div>
        </div>
    );
}
