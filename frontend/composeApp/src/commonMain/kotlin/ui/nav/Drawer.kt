package ui.nav

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalDrawerSheet
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.app_name
import hackathonschenna.composeapp.generated.resources.compose_multiplatform
import hackathonschenna.composeapp.generated.resources.settings
import org.jetbrains.compose.resources.StringResource
import org.jetbrains.compose.resources.painterResource
import org.jetbrains.compose.resources.stringResource
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
fun DrawerContent(
    onSettingsClick: () -> Unit,
    closeDrawer: () -> Unit,
) {
    ModalDrawerSheet(
        modifier = Modifier.widthIn(max = 280.dp)
    ) {
        DrawerHeader(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp)
        )

        DrawerItem(
            icon = Icons.Default.Settings,
            label = Res.string.settings,
            onClick = {
                onSettingsClick()
                closeDrawer()
            },
            modifier = Modifier
                .padding(horizontal = 12.dp)
                .testTag("settings_drawer_item"),
        )

//        DrawerItem(
//            icon = Icons.Default.RecordVoiceOver,
//            label = R.string.stt_popup,
//            onClick = {
//                onSpeechToTextPopupClick()
//                closeDrawer()
//            },
//            modifier = Modifier.padding(horizontal = 12.dp),
//        )
    }
}

@Preview
@Composable
private fun DrawerContentPreview() {
    DrawerContent(onSettingsClick = {}, closeDrawer = {})
}

@Preview
@Composable
private fun DrawerHeader(modifier: Modifier = Modifier) {
    Surface(
        shape = MaterialTheme.shapes.extraLarge,
        color = MaterialTheme.colorScheme.primary,
        modifier = modifier
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            modifier = Modifier.padding(24.dp),
        ) {
            Column(
                modifier = Modifier.weight(1.0f)
            ) {
                Text(
                    text = stringResource(Res.string.app_name),
                    style = MaterialTheme.typography.headlineMedium,
                    color = MaterialTheme.colorScheme.onPrimary,
                )
//                Text(
//                    text = stringResource(R.string.drawer_header_subtitle),
//                    style = MaterialTheme.typography.bodyMedium,
//                    color = MaterialTheme.colorScheme.onPrimary,
//                )
            }

            Icon(
                painter = painterResource(Res.drawable.compose_multiplatform),
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onPrimary,
                modifier = Modifier
                    .size(40.dp)
                    .scale(2.4f)
            )
        }
    }
}

@Composable
fun DrawerItem(
    icon: ImageVector,
    label: StringResource,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    NavigationDrawerItem(
        icon = {
            Icon(
                imageVector = icon,
                contentDescription = stringResource(label),
            )
        },
        label = {
            Text(
                text = stringResource(label),
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        },
        selected = false,
        onClick = onClick,
        modifier = modifier,
    )
}
