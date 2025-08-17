import { createContext, useContext, useState, type ReactNode } from "react";

interface ErrorContextType {
  error: string | null;
  setError: (error: string | null) => void;
}

interface ErrorProviderProps {
  children: ReactNode;
}

const ErrorContext = createContext<ErrorContextType | null>(null);

export const ErrorProvider = ({ children }: ErrorProviderProps) => {
  const [error, setError] = useState<string | null>(null);

  return (
    <ErrorContext.Provider value={{ error, setError }}>
      {children}
    </ErrorContext.Provider>
  );
};

export const useError = () => {
  const context = useContext(ErrorContext);

  if (!context) {
    throw new Error("useError must be used within an ErrorProvider");
  }

  return context;
};
