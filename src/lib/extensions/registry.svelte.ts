import type { AppExtension, ExtensionCommand, ExtensionNavItem } from '$lib/types'

class ExtensionRegistry {
    extensions = $state<AppExtension[]>([])

    register(extension: AppExtension) {
        if (this.extensions.some((item) => item.id === extension.id)) {
            return
        }

        this.extensions = [
            ...this.extensions,
            {
                enabled: true,
                ...extension,
            },
        ]
    }

    setEnabled(id: string, enabled: boolean) {
        this.extensions = this.extensions.map((extension) =>
            extension.id === id
                ? {
                      ...extension,
                      enabled,
                  }
                : extension,
        )
    }

    get enabledExtensions() {
        return this.extensions.filter((extension) => extension.enabled !== false)
    }

    get navItems(): ExtensionNavItem[] {
        return this.enabledExtensions
            .flatMap((extension) => extension.navItems ?? [])
            .sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
    }

    get commands(): ExtensionCommand[] {
        return this.enabledExtensions.flatMap(
            (extension) => extension.commands ?? [],
        )
    }
}

export const extensionRegistry = new ExtensionRegistry()