# Meta Data Export Image Date fixer

- How to [Download a copy of your information on Facebook](https://www.facebook.com/help/212802592074644)
- Pick `.json` instead of `.html`

- These scripts add the field "Created Date" back to the images on data exported from meta. 
- There was some EXIF data also available, but it wasn't common or detailed enough. I only needed the created date to sort photos and files. 
- If the created time on the image was not available, I used the upload timestamp.
- I make a huge assumption that all directories given do not start or end with `/`

Prereq: [Install Rust](https://www.rust-lang.org/learn/get-started)


## Facebook

- I had multiple exports so I merged them and ran this to one folder. 
- In one of my exports the `your_activity_across_facebook` was inside another folder. I merged that with the root directory as well. 
- I use the command `touch -t <date_time> <photo_file_path>` to update the Created Date. 

```sh
cd facebook
cargo build
cargo run -- <export_directory>   
```

<details>
  <summary>Directory Tree</summary>

- I provided the directory as a reference point for how this code was written. In a few years or months this structure might change and the code might not work anymore. 
- Some directories are empty, filtered out by tree script but they usually have a `no-data.txt` inside them.

<a href="#instagram">Jump to next Section</a>

```
./your_activity_across_facebook/
├── ads_information
│   ├── ad_preferences.json
│   ├── advertisers_using_your_activity_or_information.json
│   ├── advertisers_you've_interacted_with.json
│   ├── your_pages_mentions.json
│   └── your_sampled_locations.json
├── apps_and_websites_off_of_facebook
│   ├── connected_apps_and_websites.json
│   ├── posts_from_apps_and_websites.json
│   └── your_activity_off_meta_technologies.json
├── bug_bounty
├── comments_and_reactions
│   ├── comments.json
│   └── likes_and_reactions_1.json
├── connections
│   ├── followers
│   │   ├── people_who_followed_you.json
│   │   └── who_you've_followed.json
│   ├── friends
│   │   ├── people_you_may_know.json
│   │   ├── rejected_friend_requests.json
│   │   ├── removed_friends.json
│   │   ├── sent_friend_requests.json
│   │   └── your_friends.json
│   └── supervision
├── events
│   ├── event_invitations.json
│   ├── tickets_purchased.json
│   ├── your_event_responses.json
│   └── your_events.json
├── facebook_gaming
│   ├── bookmark_and_app_settings.json
│   ├── instant_games.json
│   ├── your_instant_game_updates.json
│   ├── your_instant_games_leaderboard_moments.json
│   └── your_latest_instant_game_scores.json
├── facebook_marketplace
│   ├── conversations_you_had_as_a_buyer.json
│   ├── your_marketplace_search_filters.json
│   └── your_transaction_survey_information.json
├── facebook_payments
│   └── payment_history.json
├── fundraisers
│   └── your_fundraiser_donations_information.json
├── groups
│   ├── group_posts_and_comments.json
│   ├── your_comments_in_groups.json
│   └── your_group_membership_activity.json
├── logged_information
│   ├── activity_messages
│   │   └── people_and_friends.json
│   ├── bonuses
│   ├── location
│   │   ├── primary_location.json
│   │   ├── primary_public_location.json
│   │   └── timezone.json
│   ├── music_recommendations
│   ├── notifications
│   │   └── notification_of_meta_privacy_policy_update.json
│   ├── other_logged_information
│   │   ├── ads_interests.json
│   │   └── consents.json
│   ├── privacy_checkup
│   ├── professional_dashboard
│   ├── search
│   │   └── your_search_history.json
│   ├── your_interactions_on_facebook
│   │   ├── recently_viewed.json
│   │   └── recently_visited.json
│   └── your_topics
│       └── people_we_think_you_should_follow.json
├── messages
│   ├── archived_threads
│   │   ├── [user_1]
│   │   │   └── message_1.json
│   │   ├── [user_2]
│   │   │   ├── message_1.json
│   │   │   └── photos
│   │   ├── [user_3]
│   │   │   └── message_1.json
│   │   ├── [user_4]
│   │   │   ├── files
│   │   │   ├── gifs
│   │   │   ├── message_1.json
│   │   │   ├── message_10.json
│   │   │   ├── message_2.json
│   │   │   ├── message_3.json
│   │   │   ├── message_4.json
│   │   │   ├── message_5.json
│   │   │   ├── message_6.json
│   │   │   ├── message_7.json
│   │   │   ├── message_8.json
│   │   │   ├── message_9.json
│   │   │   ├── photos
│   │   │   └── videos
│   │   ├── [user_5]
│   │   │   ├── message_1.json
│   │   │   └── photos
│   ├── autofill_information.json
│   ├── e2ee_cutover
│   │   ├── [user_1]
│   │   │   ├── files
│   │   │   ├── gifs
│   │   │   ├── message_1.json
│   │   │   ├── message_2.json
│   │   │   ├── message_3.json
│   │   │   ├── photos
│   │   │   └── videos
│   │   └── [user_2]
│   │       ├── audio
│   │       ├── files
│   │       │   ├── Lab7_1175417982791964.mat
│   │       │   ├── Lab7_245724923269935.mat
│   │       │   ├── MTH719Quiz7_4558176257541739.mat
│   │       │   ├── Quiz_7_324205028555484.m
│   │       ├── gifs
│   │       ├── message_1.json
│   │       ├── message_10.json
│   │       ├── message_11.json
│   │       ├── message_12.json
│   │       ├── message_13.json
│   │       ├── message_14.json
│   │       ├── message_15.json
│   │       ├── message_16.json
│   │       ├── message_17.json
│   │       ├── message_18.json
│   │       ├── message_19.json
│   │       ├── message_2.json
│   │       ├── message_20.json
│   │       ├── message_21.json
│   │       ├── message_22.json
│   │       ├── message_23.json
│   │       ├── message_24.json
│   │       ├── message_25.json
│   │       ├── message_3.json
│   │       ├── message_4.json
│   │       ├── message_5.json
│   │       ├── message_6.json
│   │       ├── message_7.json
│   │       ├── message_8.json
│   │       ├── message_9.json
│   │       ├── photos
│   │       └── videos
│   ├── filtered_threads
│   │   ├── [user_1]
│   │   │   ├── files
│   │   │   ├── message_1.json
│   │   │   ├── photos
│   │   │   └── videos
│   │   ├── [user_2]
│   │   │   ├── audio
│   │   │   ├── files
│   │   │   ├── gifs
│   │   │   ├── message_1.json
│   │   │   ├── message_10.json
│   │   │   ├── message_11.json
│   │   │   ├── message_12.json
│   │   │   ├── message_13.json
│   │   │   ├── message_14.json
│   │   │   ├── message_15.json
│   │   │   ├── message_16.json
│   │   │   ├── message_17.json
│   │   │   ├── message_18.json
│   │   │   ├── message_19.json
│   │   │   ├── message_2.json
│   │   │   ├── message_20.json
│   │   │   ├── message_21.json
│   │   │   ├── message_22.json
│   │   │   ├── message_23.json
│   │   │   ├── message_24.json
│   │   │   ├── message_25.json
│   │   │   ├── message_26.json
│   │   │   ├── message_27.json
│   │   │   ├── message_28.json
│   │   │   ├── message_29.json
│   │   │   ├── message_3.json
│   │   │   ├── message_30.json
│   │   │   ├── message_31.json
│   │   │   ├── message_32.json
│   │   │   ├── message_33.json
│   │   │   ├── message_34.json
│   │   │   ├── message_35.json
│   │   │   ├── message_36.json
│   │   │   ├── message_4.json
│   │   │   ├── message_5.json
│   │   │   ├── message_6.json
│   │   │   ├── message_7.json
│   │   │   ├── message_8.json
│   │   │   ├── message_9.json
│   │   │   ├── photos
│   │   │   │   ├── 49678708_366520640569703_3686014424612601856_n_457368821749532.webp
│   │   │   └── videos
│   │   ├── [user_3]
│   │   │   ├── files
│   │   │   ├── message_1.json
│   │   │   └── photos
│   │   ├── [user_4]
│   │   │   └── message_1.json
│   │   ├── [user_5]
│   │       ├── message_1.json
│   │       └── photos
│   ├── message_requests
│   │   ├── [user_1]
│   │   │   └── message_1.json
│   │   └── [user_2]
│   │       └── message_1.json
│   ├── messenger_contacts_you've_blocked.json
│   ├── photos
│   ├── secret_conversations.json
│   ├── stickers_used
│   └── your_cross-app_messaging_settings.json
├── meta_spark
├── navigation_bar
│   └── navigation_bar_activity.json
├── notes
├── other_activity
│   ├── notes.json
│   ├── pokes.json
│   ├── qr_code_files_you_generated.json
│   ├── reshare_education.json
│   ├── your_recently_followed_history.json
│   └── your_recently_used_emojis.json
├── pages
│   ├── admin_activity.json
│   ├── pages_and_profiles_you've_recommended.json
│   ├── pages_and_profiles_you've_unfollowed.json
│   ├── pages_and_profiles_you_follow.json
│   └── pages_you've_liked.json
├── personal_information
│   ├── facebook_accounts_center
│   │   └── accounts_center.json
│   ├── facebook_assistant
│   ├── facebook_portal
│   ├── other_personal_information
│   │   └── your_address_books.json
│   └── profile_information
│       ├── movies_and_tv.json
│       ├── profile_information.json
│       ├── profile_update_history.json
│       └── timezone.json
├── polls
│   └── polls_you_voted_on.json
├── posts
│   ├── album
│   │   ├── 0.json
│   │   ├── 1.json
│   │   ├── 10.json
│   │   ├── 11.json
│   │   ├── 2.json
│   │   ├── 3.json
│   │   ├── 4.json
│   │   ├── 5.json
│   │   ├── 6.json
│   │   ├── 7.json
│   │   ├── 8.json
│   │   └── 9.json
│   ├── media
│   │   ├── [album_1]
│   │   ├── [album_2]
│   │   ├── [album_3]
│   │   ├── stickers_used
│   │   ├── videos
│   │   └── your_posts
│   ├── your_posts__check_ins__photos_and_videos_1.json
│   ├── your_uncategorized_photos.json
│   └── your_videos.json
├── preferences
│   ├── feed
│   │   ├── controls.json
│   │   └── feed.json
│   └── preferences
│       ├── language_and_locale.json
│       ├── reels_preferences.json
│       ├── your_accessibility_settings.json
│       ├── your_device_push_settings.json
│       ├── your_fundraiser_settings.json
│       ├── your_story_highlights.json
│       ├── your_video_accessibility_settings.json
│       └── your_watch_settings.json
├── reviews
├── saved_items_and_collections
│   └── your_saved_items.json
├── security_and_login_information
│   ├── account_activity.json
│   ├── account_recoveries_without_password_changes.json
│   ├── browser_cookies.json
│   ├── email_address_verifications.json
│   ├── information_about_your_last_login.json
│   ├── ip_address_activity.json
│   ├── login_protection_data.json
│   ├── logins_and_logouts.json
│   ├── mobile_devices.json
│   ├── recognized_devices.json
│   ├── record_details.json
│   ├── where_you're_logged_in.json
│   ├── your_facebook_activity_history.json
│   └── your_recent_account_recovery_successes.json
├── shopping
├── shops
├── short_videos
├── stories
├── voting
│   ├── voting_location.json
│   └── voting_reminders.json
├── your_places
│   └── places_you've_created.json
└── your_problem_reports
```
</details>

## Instagram

- I had 2 export files and I merged them at root.
- I use the cargo package `filetime_creation` to update the created date

```sh
cd instagram
cargo build
cargo run -- <export_directory>  # If you have multiple exports, run on each dir
```

<details>
  <summary>Directory Tree</summary>

- I provided the directory as a reference point for how this code was written. In a few years or months this structure might change and the code might not work anymore. 

```
├── ads_information
│   ├── ads_and_topics
│   │   ├── accounts_you're_not_interested_in.json
│   │   ├── ads_viewed.json
│   │   ├── posts_viewed.json
│   │   ├── posts_you're_not_interested_in.json
│   │   ├── suggested_accounts_viewed.json
│   │   └── videos_watched.json
│   ├── advertising
│   │   └── no-data.txt
│   └── instagram_ads_and_businesses
│       └── advertisers_using_your_activity_or_information.json
├── apps_and_websites_off_of_instagram
│   └── apps_and_websites
│       ├── no-data.txt
│       └── your_activity_off_meta_technologies.json
├── connections
│   ├── contacts
│   │   ├── no-data.txt
│   │   └── synced_contacts.json
│   └── followers_and_following
│       ├── blocked_accounts.json
│       ├── close_friends.json
│       ├── follow_requests_you've_received.json
│       ├── followers_1.json
│       ├── following.json
│       ├── pending_follow_requests.json
│       ├── recent_follow_requests.json
│       ├── recently_unfollowed_accounts.json
│       └── removed_suggestions.json
├── logged_information
│   ├── link_history
│   │   ├── link_history.json
│   │   ├── no-data.txt
│   │   └── your_link_history_settings.json
│   ├── past_instagram_insights
│   │   └── no-data.txt
│   ├── policy_updates_and_permissions
│   │   ├── no-data.txt
│   │   └── notification_of_privacy_policy_updates.json
│   └── recent_searches
│       ├── account_searches.json
│       └── no-data.txt
├── media
│   ├── archived_posts
│   │   ├── 201712
│   │   ├── 201805
│   │   ├── 201806
│   │   ├── 201807
│   ├── other
│   ├── posts
│   │   ├── 201805
│   │   └── 202211
│   └── stories
│       ├── 202009
│       ├── 202104
│       ├── 202108
│       ├── 202308
├── personal_information
│   ├── autofill_information
│   │   └── no-data.txt
│   ├── device_information
│   │   ├── camera_information.json
│   │   └── devices.json
│   ├── digital_wallets
│   │   └── no-data.txt
│   ├── information_about_you
│   │   ├── account_based_in.json
│   │   └── possible_phone_numbers.json
│   ├── loyalty_accounts
│   │   └── no-data.txt
│   └── personal_information
│       ├── account_information.json
│       ├── linked_meta_accounts.json
│       ├── note_interactions.json
│       ├── personal_information.json
│       ├── professional_information.json
│       └── profile_changes.json
├── preferences
│   ├── media_settings
│   │   ├── comments_allowed_from.json
│   │   ├── consents.json
│   │   ├── notification_preferences.json
│   │   └── use_cross-app_messaging.json
│   └── your_topics
│       ├── no-data.txt
│       └── your_topics.json
├── security_and_login_information
│   └── login_and_account_creation
│       ├── account_privacy_changes.json
│       ├── account_status_changes.json
│       ├── last_known_location.json
│       ├── login_activity.json
│       ├── logout_activity.json
│       ├── password_change_activity.json
│       └── signup_information.json
├── your_activity_across_facebook
│   └── messages
│       └── inbox
│           ├── [id_1]
│           │   └── photos
│           ├── [id_2]
│           │   ├── photos
│           │   └── videos
│           ├── [id_3]
│           │   ├── photos
│           │   └── videos
│           ├── [id_4]
│           │   ├── audio
│           │   ├── photos
│           │   └── videos
│           ├── [id_5]
│           │   ├── audio
│           │   ├── photos
│           │   └── videos
│           └── [id_6]
│               ├── photos
│               └── videos
└── your_instagram_activity
    ├── comments
    │   ├── comments_reported.json
    │   ├── post_comments_1.json
    │   └── reels_comments.json
    ├── content
    │   ├── archived_posts.json
    │   ├── posts_1.json
    │   ├── profile_photos.json
    │   └── stories.json
    ├── events
    │   └── no-data.txt
    ├── fundraisers
    │   └── no-data.txt
    ├── gifts
    │   └── no-data.txt
    ├── guides
    │   └── no-data.txt
    ├── instagram_live
    │   └── no-data.txt
    ├── likes
    │   ├── liked_comments.json
    │   └── liked_posts.json
    ├── messages
    │   ├── inbox
    │   │   ├── [id_1]
    │   │   │   └── message_1.json
    │   │   ├── [id_2]
    │   │   │   └── message_1.json
    │   │   ├── [id_3]
    │   │   │   └── message_1.json
    │   │   ├── [id_4]
    │   │   │   ├── message_1.json
    │   │   │   └── message_2.json
    │   │   ├── [id_5]
    │   │   │   ├── message_1.json
    │   │   │   ├── message_2.json
    │   │   │   └── message_3.json
    │   │   ├── [id_6]
    │   │   │   └── message_1.json
    │   │   ├── [id_7]
    │   │   │   └── message_1.json
    │   │   ├── [id_8]
    │   │   │   └── message_1.json
    │   ├── message_requests
    │   │   ├── [id_1]
    │   │   │   └── message_1.json
    │   │   ├── [id_2]
    │   │   │   └── message_1.json
    │   │   ├── [id_3]
    │   │   │   └── message_1.json
    │   └── secret_conversations.json
    ├── meta_spark
    │   └── no-data.txt
    ├── monetization
    │   └── eligibility.json
    ├── other_activity
    │   └── no-data.txt
    ├── reports
    │   └── no-data.txt
    ├── saved
    │   ├── saved_collections.json
    │   └── saved_posts.json
    ├── shopping
    │   ├── no-data.txt
    │   ├── recently_viewed_items.json
    │   └── wishlist_items.json
    ├── story_sticker_interactions
    │   ├── countdowns.json
    │   ├── emoji_sliders.json
    │   ├── polls.json
    │   ├── questions.json
    │   ├── quizzes.json
    │   └── story_likes.json
    ├── subscriptions
    │   └── no-data.txt
    └── threads
        ├── follow_requests_you've_received.json
        ├── followers.json
        ├── following.json
        ├── no-data.txt
        ├── pending_follow_requests.json
        ├── personal_information.json
        └── recent_follow_requests.json
```
</details>