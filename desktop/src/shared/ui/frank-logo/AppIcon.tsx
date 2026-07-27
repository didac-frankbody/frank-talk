import { cn } from "@/shared/lib/cn";
import { useTheme } from "@/shared/theme/ThemeProvider";

/**
 * The frank talk app icon, in the variant that suits the active theme.
 *
 * Two variants ship: white ground with an ink mark and ink border (the
 * default — it is also the packaged app icon in `src-tauri/icons/`), and its
 * inverse, ink ground with a white mark and white border.
 *
 * Unlike the wordmark, this one *is* theme-aware. Both variants are a solid
 * tile, so the wrong one reads as a glaring white block on ink or a heavy black
 * block on the pale page. Each carries a border in the opposite value, so the
 * theme-matched variant keeps a defined edge against its own surface instead of
 * melting into it.
 *
 * `@2x` / `@3x` are 112px and 168px — the sizes every current slot renders at.
 */
const VARIANTS = {
  light: { src: "/app-icon@2x.png", x3: "/app-icon@3x.png" },
  dark: { src: "/app-icon-dark@2x.png", x3: "/app-icon-dark@3x.png" },
} as const;

/**
 * `src` / `srcSet` for the app icon under the active theme.
 *
 * Use this where a component takes an image URL as a plain string prop and
 * cannot host an `<img>` of its own (the pairing QR's centre image, avatar
 * components). Everywhere else, prefer {@link AppIcon}.
 */
export function useAppIcon(): { src: string; srcSet: string } {
  const { isDark } = useTheme();
  const variant = isDark ? VARIANTS.dark : VARIANTS.light;
  return {
    src: variant.src,
    srcSet: `${variant.src} 1x, ${variant.x3} 2x`,
  };
}

/** The app icon as an `<img>`, sized by the caller. */
export function AppIcon({
  alt = "frank talk",
  className,
}: {
  alt?: string;
  className?: string;
}) {
  const { src, srcSet } = useAppIcon();

  return (
    <img
      alt={alt}
      className={cn("block select-none", className)}
      draggable={false}
      src={src}
      srcSet={srcSet}
    />
  );
}
