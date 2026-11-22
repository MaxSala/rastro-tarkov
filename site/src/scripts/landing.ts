// The landing page's few behaviours. Everything works without them: the page is complete as HTML
// and CSS, this only adds the feature tabs, the reveal of the file name and small conveniences.
const still = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

// Header: a hairline under it once the page has scrolled.
const top = document.querySelector<HTMLElement>(".top");
const onScroll = () => top?.toggleAttribute("data-scrolled", window.scrollY > 8);
window.addEventListener("scroll", onScroll, { passive: true });
onScroll();

// Language menu: close on a click outside or Escape.
const menu = document.querySelector<HTMLDetailsElement>("[data-menu]");
document.addEventListener("click", (e) => {
  if (menu?.open && !menu.contains(e.target as Node)) menu.open = false;
});
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && menu?.open) {
    menu.open = false;
    menu.querySelector("summary")?.focus();
  }
});

// The file name draws its parts in order the first time it comes into view, and the hero's
// screenshot plays once the app window is actually on screen.
const reveals = document.querySelectorAll<HTMLElement>("[data-reveal]");
if (!still && "IntersectionObserver" in window) {
  const io = new IntersectionObserver(
    (entries) => {
      for (const e of entries) {
        if (!e.isIntersecting) continue;
        e.target.setAttribute("data-in", "");
        io.unobserve(e.target);
      }
    },
    { threshold: 0.45 },
  );
  [...reveals, ...document.querySelectorAll<HTMLElement>(".sheet")].forEach((el) => {
    el.setAttribute("data-armed", "");
    io.observe(el);
  });
}

// Feature tabs. They advance by themselves while in view, until the visitor picks one.
const showcase = document.querySelector<HTMLElement>("[data-showcase]");
if (showcase) {
  const tabs = [...showcase.querySelectorAll<HTMLButtonElement>("[role=tab]")];
  const panels = [...showcase.querySelectorAll<HTMLElement>("[role=tabpanel]")];
  let current = 0;

  const select = (i: number, focus = false) => {
    current = (i + tabs.length) % tabs.length;
    tabs.forEach((tab, k) => {
      const on = k === current;
      tab.setAttribute("aria-selected", String(on));
      tab.tabIndex = on ? 0 : -1;
      panels[k]?.toggleAttribute("data-active", on);
    });
    if (focus) tabs[current]?.focus();
    // On narrow screens the tabs are a row that scrolls sideways: keep the active one in sight.
    const bar = tabs[current]?.parentElement;
    const tab = tabs[current];
    if (bar && tab && bar.scrollWidth > bar.clientWidth) {
      bar.scrollTo({ left: tab.offsetLeft - 16, behavior: still ? "auto" : "smooth" });
    }
  };
  const stop = () => showcase.removeAttribute("data-auto");

  tabs.forEach((tab, i) => {
    tab.addEventListener("click", () => {
      stop();
      select(i);
    });
    tab.addEventListener("keydown", (e) => {
      const step = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 }[e.key];
      const jump = e.key === "Home" ? 0 : e.key === "End" ? tabs.length - 1 : null;
      if (step === undefined && jump === null) return;
      e.preventDefault();
      stop();
      select(jump ?? current + (step ?? 0), true);
    });
    // The bar under the active tab is a CSS animation; when it ends, move on.
    tab.querySelector(".tab-progress")?.addEventListener("animationend", () => {
      if (showcase.hasAttribute("data-auto")) select(current + 1);
    });
  });
  showcase.addEventListener("focusin", (e) => {
    if ((e.target as HTMLElement).matches("[role=tab]") && e.target !== tabs[current]) stop();
  });

  showcase.setAttribute("data-ready", "");
  if (!still && "IntersectionObserver" in window) {
    new IntersectionObserver(
      ([e]) => showcase.toggleAttribute("data-visible", Boolean(e?.isIntersecting)),
      { threshold: 0.35 },
    ).observe(showcase);
    showcase.setAttribute("data-auto", "");
  }
}
