package ui.settings

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.QuestionAnswer
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
fun SettingsItem(
    title: String,
    modifier: Modifier = Modifier,
    icon: ImageVector? = null,
    description: String? = null,
    content: (@Composable () -> Unit)? = null,
) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = modifier
            .padding(horizontal = 16.dp, vertical = 12.dp),
    ) {
        if (icon != null) {
            Icon(
                imageVector = icon,
                contentDescription = title,
            )
            Spacer(modifier = Modifier.width(24.dp))
        }
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .weight(1.0f)
        ) {
            Text(
                text = title,
                style = MaterialTheme.typography.bodyLarge,
                lineHeight = MaterialTheme.typography.bodyMedium.lineHeight,
                fontWeight = FontWeight.Medium,
            )
            if (description != null) {
                Spacer(modifier = Modifier.height(3.dp))
                Text(
                    text = description,
                    style = MaterialTheme.typography.bodyMedium,
                )
            }
        }
        if (content != null) {
            Spacer(modifier = Modifier.width(16.dp))
            content()
        }
    }
}

@Composable
@Preview
private fun SettingsItemPreview() {
    val loremIpsumShort = { "Lorem ipsum dolor sit" }
    val loremIpsumLong = { "Lorem ipsum dolor sit amet, consectetur adipiscing elit" }

    Column {
        SettingsItem(
            title = "Test",
            modifier = Modifier.clickable {  },
        )
        SettingsItem(
            title = "Input and output methods",
            icon = Icons.Default.QuestionAnswer,
        )
        SettingsItem(
            title = loremIpsumLong(),
            description = "A nice description",
        )
        SettingsItem(
            title = loremIpsumShort(),
            icon = Icons.Default.QuestionAnswer,
            description = loremIpsumLong(),
            modifier = Modifier.clickable {  },
        )
        var checked1 by remember { mutableStateOf(false) }
        SettingsItem(
            title = loremIpsumLong(),
            content = {
                Switch(checked = checked1, onCheckedChange = { checked1 = it })
            }
        )
        var checked2 by remember { mutableStateOf(true) }
        SettingsItem(
            title = loremIpsumShort(),
            icon = Icons.Default.QuestionAnswer,
            modifier = Modifier.clickable { checked2 = !checked2 },
            content = {
                Switch(checked = checked2, onCheckedChange = { checked2 = it })
            }
        )
        var checked3 by remember { mutableStateOf(true) }
        SettingsItem(
            title = "Test",
            description = loremIpsumShort(),
            modifier = Modifier.clickable { checked3 = !checked3 },
            content = {
                Checkbox(checked = checked3, onCheckedChange = { checked3 = it })
            }
        )
        var checked4 by remember { mutableStateOf(false) }
        SettingsItem(
            title = "Input and output methods",
            icon = Icons.Default.QuestionAnswer,
            description = "A nice long nice long nice long nice long nice long nice long nice long description",
            content = {
                Checkbox(checked = checked4, onCheckedChange = { checked4 = it })
            }
        )
    }
}
