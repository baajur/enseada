import Vue, { ComponentOptions } from 'vue';
import { IdMapFn, Page } from "../types";
import { pageToOffset } from "../http/Page";
import { svcGetter } from "../utils";

interface ListData<T> {
  limit: number,
  loading: boolean,
  checked: T[],
  page: Page<T>
}

interface FactoryParams<T> {
  name: string,
  service: string,
  mapId: IdMapFn<T>,
}

function factory<T>({ name, service, mapId }: FactoryParams<T>): ComponentOptions<Vue> {
  const svc = svcGetter(`$${service}`);
  return {
    data(): ListData<T> {
      return {
        limit: 25,
        loading: false,
        checked: [],
        page: {
          count: 0,
          total: 0,
          offset: 0,
          limit: 0,
          items: []
        },
      };
    },
    computed: {
      items() {
        return this.page.items
      },
      count() {
        return this.page.count
      },
      offset() {
        return this.page.offset
      }
    },
    methods: {
      async fetch(offset: number = 0) {
        this.loading = true
        this.page = await svc(this).list({ offset, limit: this.limit })
        this.loading = false
      },
      onPageChange(page) {
        return this.fetch(pageToOffset(page, this.limit)).catch((err) => this.$emit('error', err))
      },
      async remove() {
        try {
          const ids = this.checked.map(mapId);
          const promises = ids.map((id) => svc(this).remove(id))
          await Promise.all(promises)
          ids.forEach((id) => {
            this.$buefy.notification.open({
              message: `Deleted ${name} ${id}`,
              type: 'is-warning',
              position: 'is-bottom-right',
              duration: 10000
            })
          });

          return this.fetch(this.offset)
        } catch (err) {
          return this.$emit('error', err)
        }
      },
      mapId,
    },
    async created() {
      return this.fetch().catch((err) => this.$emit('error', err))
    },
  }
}

export default factory;