package ui.dummy

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.dummy
import org.jetbrains.compose.resources.stringResource
import ui.nav.Route

@Composable
fun DummyScreen(number: Int, navigationIcon: @Composable () -> Unit) {
    Scaffold(
        topBar = {
            @OptIn(ExperimentalMaterial3Api::class)
            TopAppBar(
                title = { Text(stringResource(Res.string.dummy)) },
                navigationIcon = navigationIcon
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier.padding(paddingValues)
        ) {
            Text("Number is $number")
        }
    }
}
