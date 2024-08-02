package ui.settings

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.BookmarkAdded
import androidx.compose.material.icons.filled.BookmarkRemove
import androidx.compose.material.icons.filled.Dns
import androidx.compose.material.icons.filled.Percent
import androidx.compose.material.icons.filled.WaterDrop
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.russhwolf.settings.get
import com.russhwolf.settings.set
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.setting_server_url_description
import hackathonschenna.composeapp.generated.resources.setting_server_url_title
import hackathonschenna.composeapp.generated.resources.settings
import org.jetbrains.compose.resources.stringResource
import settings

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(navigationIcon: @Composable () -> Unit) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(stringResource(Res.string.settings)) },
                navigationIcon = navigationIcon
            )
        }
    ) { paddingValues ->
        LazyColumn(
            modifier = Modifier.padding(paddingValues)
        ) {
            item {
                var settingValue by rememberSaveable { mutableStateOf(settings.getString(
                    SettingsKeys.SERVER_URL, SettingsKeys.SERVER_URL_DEFAULT)) }
                StringSetting(
                    title = stringResource(Res.string.setting_server_url_title),
                    icon = Icons.Default.Dns,
                    description = stringResource(Res.string.setting_server_url_description),
                    descriptionWhenEmpty = stringResource(Res.string.setting_server_url_description),
                ).Render(settingValue) {
                    settingValue = it
                    settings.putString(SettingsKeys.SERVER_URL, it)
                }
            }

            item {
                var settingValue by rememberSaveable { mutableStateOf(false) }
                BooleanSetting(
                    title = "Test setting",
                    icon = Icons.Default.Percent,
                    descriptionOn = "Enabled, tap to disable",
                    descriptionOff = "Disabled, tap to enable"
                ).Render(settingValue) { settingValue = it }
            }

            item {
                var settingValue by rememberSaveable { mutableStateOf(true) }
                ListSetting(
                    title = "List of things",
                    icon = Icons.Default.WaterDrop,
                    possibleValues = listOf(
                        ListSetting.Value(true, "True!", icon = Icons.Default.BookmarkAdded),
                        ListSetting.Value(false, "False :-( ".repeat(20), icon = Icons.Default.BookmarkRemove),
                    )
                ).Render(settingValue) { settingValue = it }
            }
        }
    }
}
