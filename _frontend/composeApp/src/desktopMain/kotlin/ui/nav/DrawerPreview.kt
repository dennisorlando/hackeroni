package ui.nav

import androidx.compose.desktop.ui.tooling.preview.Preview
import androidx.compose.runtime.Composable

@Preview
@Composable
private fun DrawerContentPreview() {
    DrawerContent(onSettingsClick = {}, onDummyClick = {}, onMapClick = {}, closeDrawer = {})
}
