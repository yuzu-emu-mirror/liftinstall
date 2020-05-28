<template>
    <div class="column has-padding">
            <h4 class="subtitle">{{ $t('modify.title') }}</h4>

            <b-button class="is-dark is-medium" v-on:click="update">
                {{ $t('modify.update') }}
            </b-button>
            <br />
            <br />

            <b-button class="is-dark is-medium" v-on:click="modify_packages">
                {{ $t('modify.modify') }}
            </b-button>
            <br />
            <br />

            <a class="button is-dark is-medium" v-on:click="repair_packages">
                {{ $t('modify.repair') }}
            </a>
            <br />
            <br />

            <a class="button is-dark is-medium" v-on:click="prepare_uninstall">
                {{ $t('modify.uninstall') }}
            </a>
            <br />
            <br />

            <a class="button is-dark is-medium" v-on:click="view_files">
                {{ $t('modify.view_local_files') }}
            </a>
    </div>
</template>

<script>
export default {
  name: 'ModifyView',
  data: function () {
    return {}
  },
  methods: {
    update: function () {
      this.$router.push('/install/update')
    },
    modify_packages: function () {
      this.$router.push('/packages')
    },
    repair_packages: function () {
      this.$router.push({ name: 'packages', params: { repair: true } })
    },
    prepare_uninstall: function () {
      this.$buefy.dialog.confirm({
        title: this.$t('modify.uninstall'),
        message: this.$t('modify.prompt', {'name': this.$root.$data.attrs.name}),
        confirmText: this.$t('modify.prompt_confirm', {'name': this.$root.$data.attrs.name}),
        type: 'is-danger',
        hasIcon: true,
        onConfirm: this.uninstall
      })
    },
    uninstall: function () {
      this.$router.push('/install/uninstall')
    },
    view_files: function () {}
  }
}
</script>
