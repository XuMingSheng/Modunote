import { useState } from "react";
import { FunctionalSidebar } from "./components/FuncitonalSidebar";
import { Workplace } from "./pages/Workplace";

function App() {
  const [page, setPage] = useState<string>("workplace");
  // const [ping, setPing] = useState<string>("...");

  // useEffect(() => {
  //   const apiUrl = import.meta.env.VITE_API_URL;
  //   fetch(`${apiUrl}/api/ping`)
  //     .then((res) => res.text())
  //     .then(setPing)
  //     .catch(() => setPing("backend offline"));
  // }, []);

  return (
    <div className="flex min-h-screen bg-gray-100">
      <FunctionalSidebar active={page} onChange={setPage} />
      <main className="flex-1 ml-14 transition-all duration-300">
        {page === "workplace" && <Workplace />}
        {page === "graph" && <div>Graph Page (To be implemented)</div>}
        {page === "notes" && <div>Notes Page (To be implemented)</div>}
      </main>
    </div>
  );
}

export default App;
