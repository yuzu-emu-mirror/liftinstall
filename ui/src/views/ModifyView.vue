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

            <b-button class="is-dark is-medium" v-on:click="prepare_uninstall">
                {{ $t('modify.uninstall') }}
            </b-button>

            <div class="modal is-active" v-if="show_uninstall">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{{ $t('modify.prompt', {'name': $root.$data.attrs.name}) }}</p>
                    </header>
                    <footer class="modal-card-foot">
                        <button class="button is-danger" v-on:click="uninstall">{{ $t('yes') }}</button>
                        <button class="button" v-on:click="cancel_uninstall">{{ $t('no') }}</button>
                    </footer>
                </div>
            </div>
    </div>
</template>

<script>
export default {
  name: 'ModifyView',
  data: function () {
    return {
      show_uninstall: false
    }
  },
  methods: {
    update: function () {
      this.$router.push('/install/update')
    },
    modify_packages: function () {
      this.$router.push('/packages')
    },
    prepare_uninstall: function () {
      this.show_uninstall = true
    },
    cancel_uninstall: function () {
      this.show_uninstall = false
    },
    uninstall: function () {
      this.$router.push('/install/uninstall')
    }
  }
}
</script>
