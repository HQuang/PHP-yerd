<script setup lang="ts">
import {
  ArrowUpRight,
  FolderOpen,
  FolderTree,
  Link2,
  Lock,
  LockOpen,
  Pencil,
} from "lucide-vue-next";

import Button from "@/components/ui/Button.vue";
import Spinner from "@/components/ui/Spinner.vue";
import { openTitle, siteUrl } from "@/lib/siteUrl";
import { openInBrowser, openPath } from "@/ipc/client";
import { openWpAdmin } from "@/lib/wpAdmin";
import type { SiteEntry, StatusReport } from "@/ipc/types";

const props = defineProps<{
  site: SiteEntry;
  report: StatusReport | null;
  tld: string;
  /** Whether a mutation targeting this site is in flight (shows a spinner). */
  busy?: boolean;
}>();

const emit = defineEmits<{
  edit: [site: SiteEntry];
  toggleSecure: [site: SiteEntry];
}>();

/** The served sub-directory label ("/" when the project root is served). */
function servedLabel(s: SiteEntry): string {
  return s.web_subpath && s.web_subpath !== "" ? s.web_subpath : "/";
}

/** The site's primary domain FQDN: its `primary_domain` when set, else the
 *  default `{name}.{tld}` apex. */
function displayHost(s: SiteEntry): string {
  return s.primary_domain ?? `${s.name}.${props.tld}`;
}

/** Number of additional domains beyond the primary (0 for a default site). */
function extraDomainCount(s: SiteEntry): number {
  return s.domains && s.domains.length > 1 ? s.domains.length - 1 : 0;
}
</script>

<template>
  <div
    class="group rounded-lg border bg-card p-4 shadow-sm transition-colors hover:border-brand/40"
  >
    <div class="flex items-start justify-between gap-2">
      <div class="min-w-0">
        <button
          class="flex max-w-full items-center gap-1.5 font-mono text-sm font-medium hover:text-brand"
          :title="openTitle(site, report)"
          @click="openInBrowser(siteUrl(site, report))"
        >
          <span class="truncate">{{ displayHost(site) }}</span>
          <ArrowUpRight class="size-3 shrink-0 opacity-70" />
          <span
            v-if="extraDomainCount(site) > 0"
            class="shrink-0 rounded bg-muted px-1 text-[10px] font-normal text-muted-foreground"
            :title="site.domains?.join(', ')"
          >+{{ extraDomainCount(site) }}</span>
        </button>
        <p
          v-if="site.apex_shadowed_by"
          class="mt-0.5 text-[11px] text-amber-600 dark:text-amber-500"
        >
          {{ site.name }}.{{ tld }} is served by "{{ site.apex_shadowed_by }}"
        </p>
        <button
          class="mt-1 flex max-w-full items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
          :title="`Reveal ${site.document_root}`"
          @click="openPath(site.document_root)"
        >
          <FolderOpen class="size-3 shrink-0" />
          <span class="truncate font-mono">{{ site.document_root }}</span>
        </button>
      </div>

      <div class="flex shrink-0 items-center">
        <Spinner v-if="busy" class="size-4" />
        <!-- Every per-site action lives in the details sidebar this opens; the
             card keeps only the inline shortcuts (open, HTTPS, and the WPA
             one-click login chip when the site has it enabled). -->
        <Button
          variant="ghost"
          size="icon"
          :aria-label="`Edit ${site.name}`"
          :title="`Edit ${site.name}`"
          @click="emit('edit', site)"
        >
          <Pencil class="size-4" />
        </Button>
      </div>
    </div>

    <!-- meta chips -->
    <div class="mt-3 flex flex-wrap items-center gap-1.5">
      <span
        class="inline-flex items-center rounded-md bg-muted px-1.5 py-0.5 font-mono text-[11px] font-medium text-muted-foreground"
      >
        PHP {{ site.php }}
      </span>
      <button
        type="button"
        :disabled="busy"
        :aria-label="site.secure ? 'Serve over HTTP' : 'Serve over HTTPS'"
        :title="site.secure ? 'Serving over HTTPS - click to switch to HTTP' : 'Serving over HTTP - click to switch to HTTPS'"
        class="inline-flex items-center gap-1 rounded-md px-1.5 py-0.5 text-[11px] font-medium transition-opacity hover:opacity-70 disabled:cursor-not-allowed disabled:opacity-50"
        :class="site.secure ? 'bg-success/10 text-success' : 'bg-muted text-muted-foreground'"
        @click="emit('toggleSecure', site)"
      >
        <Lock v-if="site.secure" class="size-3" />
        <LockOpen v-else class="size-3" />
        {{ site.secure ? "HTTPS" : "HTTP" }}
      </button>
      <span
        v-if="site.web_subpath"
        class="inline-flex items-center rounded-md bg-muted px-1.5 py-0.5 font-mono text-[11px] text-muted-foreground"
        :title="`Serves ${servedLabel(site)} as the document root`"
      >
        /{{ servedLabel(site) }}
      </span>
      <span
        v-if="site.is_wordpress"
        class="inline-flex items-center rounded-md bg-brand/10 px-1.5 py-0.5 text-[11px] font-medium text-brand"
        title="WordPress site"
      >
        WP
      </span>
      <button
        v-if="site.wp_auto_login"
        type="button"
        class="inline-flex items-center rounded-md bg-warning/10 px-1.5 py-0.5 text-[11px] font-medium text-warning transition-opacity hover:opacity-70"
        :title="`One-click login enabled - signs in as ${site.wp_auto_login_user || 'the site admin'}`"
        @click="openWpAdmin(site, report)"
      >
        WPA
      </button>
      <span class="ml-auto inline-flex items-center gap-1 text-[11px] text-muted-foreground">
        <Link2 v-if="site.kind === 'linked'" class="size-3" />
        <FolderTree v-else class="size-3" />
        {{ site.kind }}
      </span>
    </div>
  </div>
</template>
