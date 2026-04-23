import GlassCard from "../components/GlassCard";
import StudentSearch from "./StudentSearch";

export default function Admin() {
  return (
    <main className="mx-auto max-w-2xl px-5 flex items-center justify-center min-h-screen ">
      <GlassCard className="p-8">
        <h1 className="text-2xl font-bold text-light mb-1">Admin Panel</h1>
        <p className="text-sm text-secondary/50 mb-8">
          Manage student accounts
        </p>
        <h2 className="text-sm font-semibold text-secondary/70 uppercase tracking-wider mb-4">
          Student Lookup
        </h2>
        <StudentSearch />
      </GlassCard>
    </main>
  );
}
