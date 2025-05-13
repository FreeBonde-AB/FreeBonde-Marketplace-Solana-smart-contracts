import React, { createContext, useState, useEffect, useContext } from 'react';

interface FreeBondeBalanceContextType {
  freeBondeBalance: number;
  setFreeBondeBalance: React.Dispatch<React.SetStateAction<number>>;
}

// Create the Context with a default undefined value
const FreeBondeBalanceContext = createContext<FreeBondeBalanceContextType | undefined>(undefined);

// Custom hook to use the context, providing a more convenient way to access it
const useFreeBondeBalance = () => {
  const context = useContext(FreeBondeBalanceContext);
  if (context === undefined) {
    throw new Error('useFreeBondeBalance must be used within a FreeBondeBalanceProvider');
  }
  return context;
};

interface FreeBondeBalanceProviderProps {
  children: React.ReactNode;
}

// Provider component
const FreeBondeBalanceProvider: React.FC<FreeBondeBalanceProviderProps> = ({ children }) => {
  const [freeBondeBalance, setFreeBondeBalance] = useState<number>(() => {
    // Initialize state from localStorage when the Provider mounts
    const savedBalance = localStorage.getItem('freeBondeBalance');
    if (savedBalance !== null) {
      const parsedBalance = parseInt(savedBalance, 10);
      return parsedBalance;
    }
    return 0;
  });

  // Save FreeBonde balance to localStorage whenever it changes
  useEffect(() => {
    localStorage.setItem('freeBondeBalance', freeBondeBalance.toString());
    console.log('FreeBonde balance changed and saved to localStorage:', freeBondeBalance);
  }, [freeBondeBalance]);

  return (
    <FreeBondeBalanceContext.Provider value={{ freeBondeBalance, setFreeBondeBalance }}>
      {children}
    </FreeBondeBalanceContext.Provider>
  );
};

export { FreeBondeBalanceProvider, useFreeBondeBalance };