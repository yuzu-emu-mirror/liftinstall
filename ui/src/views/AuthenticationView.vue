<template>
  <div class="column has-padding">
    <section>
    <b-message type="is-info" :active.sync="browser_opened">
      {{ $t('auth.page_opened') }}
    </b-message>
    <b-message type="is-info" :active.sync="show_header">
      {{ $t('auth.page_header') }}
    </b-message>
    <div>
      If you are a subscriber, <a v-on:click="launch_browser('https://profile.yuzu-emu.org/')">click here to link your yuzu-emu.org account</a>
      <br>
      If you are not already a subscriber, <a v-on:click="launch_browser('https://www.patreon.com/join/yuzuteam/checkout?rid=2822069')">click here to become one</a>
    </div>
    </section>

    <br>

    <section>
      <p>{{ $t('auth.token') }}</p>
      <b-field>
        <b-input type="text" v-model="combined_token" placeholder="Token" id="token" style='width: 50em;'></b-input>
        <p class="control">
          <button class="button is-info" v-on:click="paste">{{ $t('auth.paste') }}</button>
        </p>
      </b-field>
    </section>

    <br>

    <section>

    <b-message type="is-danger" :active.sync="invalid_token">
      {{ $t('auth.login_failed') }}
    </b-message>

    <b-message type="is-danger" :active.sync="invalid_login">
      {{ $t('auth.login_failed') }}
    </b-message>

    <b-message type="is-danger" :active.sync="unlinked_patreon">
      Your credentials are valid, but you still need to link your patreon!
      If this is an error, then <a v-on:click="launch_browser('https://profile.yuzu-emu.org/')">click here to link your yuzu-emu.org account</a>
    </b-message>

    <b-message type="is-danger" :active.sync="no_subscription">
      Your patreon is linked, but you are not a current subscriber!
      <a v-on:click="launch_browser('https://www.patreon.com/join/yuzuteam/checkout?rid=2822069')">Log into your patreon account</a> and support the project!
    </b-message>

    <b-message type="is-danger" :active.sync="tier_not_selected">
      Your patreon is linked, and you are supporting the project, but you must first join the Early Access reward tier!
      <a v-on:click="launch_browser('https://www.patreon.com/join/yuzuteam/checkout?rid=2822069')">Log into your patreon account</a> and choose to back the Early Access reward tier.
    </b-message>
    </section>

    <div class="is-left-floating is-bottom-floating">
      <p class="control">
        <a class="button is-medium" v-on:click="go_back">{{ $t('back') }}</a>
      </p>
    </div>

    <div class="is-right-floating is-bottom-floating">
      <p class="control">
        <b-button type="is-link is-medium" :loading="loading" v-on:click="verify_token">{{ $t('auth.verify') }}</b-button>
      </p>
    </div>
  </div>
</template>

<script>
const confetti = require('canvas-confetti')
export default {
  name: 'AuthenticationView',
  created: function () {
    // If they are already authenticated when this page is loaded,
    // then we can asssume they are "clicking here for more details" and should show the appropriate error message
    if (this.$root.is_authenticated) {
      this.verification_opened = true
    }
  },
  data: function () {
    return {
      browser_opened: false,
      verification_opened: false,
      invalid_token: false,
      loading: false
    }
  },
  computed: {
    show_header: function () {
      return !this.browser_opened && !this.verification_opened && !this.invalid_token
    },
    invalid_login: function () {
      return this.verification_opened && !this.$root.is_authenticated
    },
    unlinked_patreon: function () {
      return this.verification_opened && this.$root.is_authenticated && !this.$root.is_linked
    },
    no_subscription: function () {
      return this.verification_opened && this.$root.is_linked && !this.$root.is_subscribed
    },
    tier_not_selected: function () {
      return this.verification_opened && this.$root.is_linked && this.$root.is_subscribed && !this.$root.has_reward_tier
    },
    combined_token: {
      // getter
      get: function () {
        if (this.$root.$data.username && this.$root.$data.token) {
          return btoa(this.$root.$data.username + ':' + this.$root.$data.token)
        }
        return ''
      },
      // setter
      set: function (newValue) {
        try {
          const split = atob(newValue).split(':')
          this.$root.$data.username = split[0]
          this.$root.$data.token = split[1]
          this.invalid_token = false
        } catch (e) {
          this.invalid_token = true
        }
      }
    }
  },
  methods: {
    go_back: function () {
      this.$router.go(-1)
    },
    paste: function () {
      document.getElementById('token').focus()
      document.execCommand('paste')
    },
    launch_browser: function (url) {
      const that = this
      this.$http.post('/api/open-browser', {
        url: url
      }).then(function () {
        // only open the browser opened message if there isn't an error message currently
        if (!that.verification_opened) {
          that.browser_opened = true
        }
      }).catch(function () {})
    },
    verify_token: function () {
      this.loading = true
      this.browser_opened = false
      this.$root.check_authentication(this.success, this.error)
    },
    success: function () {
      if (this.$root.has_reward_tier) {
        // show a success animation
        confetti.default({
          particleCount: 200,
          spread: 90,
          origin: { x: 0.6 }
        })
        // if they are eligible, go back to the select package page
        this.$router.go(-1)
        return
      }
      // They aren't currently eligible for the release, so display the error message
      this.verification_opened = true
      this.loading = false
    },
    error: function () {
      this.loading = false
      this.verification_opened = true
    }
  }
}
</script>
