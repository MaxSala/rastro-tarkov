/** Pantalla completa para estados sin salida desde la propia web (sin token, sesión caducada). */
export function FullMessage({ title, body }: { title: string; body: string }) {
  return (
    <main className="grid h-full place-items-center p-6">
      <div className="grid max-w-sm gap-2">
        <h1 className="text-lg font-medium">{title}</h1>
        <p className="text-sm text-fg-soft">{body}</p>
      </div>
    </main>
  );
}
