<template>
  <div class="column has-padding">
    <b-message type="is-info" :active.sync="browser_opened">
      Page opened! Check your default browser for the page, and follow the instructions there to link your patreon account.
      When you are done, enter the username and token below.
    </b-message>
    <p>
      Before you can install this Early Access, you need to verify your account.
      <a v-on:click="launch_browser('https://profile.yuzu-emu.org/')">Click here to link your yuzu-emu.org account</a>
      and paste the token below.
    </p>

    <br>

    <div class="control">
      <label for="username">Username</label>
      <input class="input" type="text" v-model="$root.$data.username" placeholder="Username" id="username">
    </div>
    <div class="control">
      <label for="token">Token</label>
      <input class="input" type="password" v-model="$root.$data.token" placeholder="Token" id="token">
    </div>

    <br>

    <b-message type="is-danger" :active.sync="invalid_login">
      Login failed!
      Double check that your username and token are correct and try again
    </b-message>

    <b-message type="is-danger" :active.sync="unlinked_patreon">
      Your credentials are valid, but you still need to link your patreon!
      If this is an error, then <a v-on:click="launch_browser('https://profile.yuzu-emu.org/external/patreon/connect/')">click here to link your yuzu-emu.org account</a>
    </b-message>

    <b-message type="is-danger" :active.sync="no_subscription">
      Your patreon is linked, but you are not a current subscriber.
      <a v-on:click="launch_browser('https://profile.yuzu-emu.org/')">Log into your patreon account</a> and support the project!
    </b-message>

    <b-message type="is-danger" :active.sync="tier_not_selected">
      Your patreon is linked, and you are supporting the project, but you must first join the Early Access reward tier!
      <a v-on:click="launch_browser('https://profile.yuzu-emu.org/')">Log into your patreon account</a> and choose to back the Early Access reward tier.
    </b-message>

    <div class="is-left-floating is-bottom-floating">
      <p class="control">
        <a class="button is-medium" v-on:click="go_back">Back</a>
      </p>
    </div>

    <div class="is-right-floating is-bottom-floating">
      <p class="control">
        <a class="button is-dark is-medium" v-on:click="verify_token">Verify Token</a>
      </p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'AuthenticationView',
  created: function() {
    // If they are already authenticated when this page is loaded,
    // then we can asssume they are "clicking here for more details" and should show the appropriate error message
    if (this.$root.is_authenticated) {
      this.verification_opened = true;
    }
  },
  data: function() {
    return {
      browser_opened: false,
      verification_opened: false,
    }
  },
  computed: {
    invalid_login: function() {
      return this.verification_opened && !this.$root.is_authenticated;
    },
    unlinked_patreon: function() {
      return this.verification_opened && this.$root.is_authenticated && !this.$root.is_linked;
    },
    no_subscription: function() {
      return this.verification_opened && this.$root.is_linked && !this.$root.is_subscribed;
    },
    tier_not_selected: function() {
      return this.verification_opened && this.$root.is_linked && this.$root.is_subscribed && !this.$root.has_reward_tier;
    }
  },
  methods: {
    go_back: function () {
      this.$router.go(-1)
    },
    launch_browser: function(url) {
      const that = this;
      let app = this.$root;
      app.ajax('/api/open-browser', function (e) {
        // only open the browser opened message if there isn't an error message currently
        if (!that.verification_opened) {
          that.browser_opened = true;
        }
      }, function (e) {}, {
        "url": url,
      });
    },
    verify_token: function() {
      this.browser_opened = false;
      this.$root.check_authentication(this.success, this.error);
    },
    success: function() {
      // if they are eligible, go back to the select package page
      if (this.$root.has_reward_tier) {
        this.$router.go(-1);
        return;
      }
      // They aren't currently eligible for the release, so display the error message
      this.verification_opened = true;
    },
    error: function() {
      this.verification_opened = true;
    }
  }
}
</script>