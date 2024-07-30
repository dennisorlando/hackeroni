import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.navigation.NavHost
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import coil3.ImageLoader
import coil3.annotation.ExperimentalCoilApi
import coil3.compose.AsyncImage
import coil3.compose.setSingletonImageLoaderFactory
import coil3.request.crossfade
import coil3.util.DebugLogger
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.app_name
import org.jetbrains.compose.resources.stringResource
import org.jetbrains.compose.ui.tooling.preview.Preview
import ui.nav.Home

import ui.theme.AppTheme

@OptIn(ExperimentalCoilApi::class, ExperimentalMaterial3Api::class)
@Composable
@Preview
fun App() {
    setSingletonImageLoaderFactory { context ->
        ImageLoader.Builder(context)
            .crossfade(true)
            .logger(DebugLogger())
            .build()
    }

    val navController = rememberNavController()

    AppTheme {
        NavHost(navController = navController, startDestination = Home) {
            composable<Home> {
                Scaffold(
                    topBar = {
                        TopAppBar(
                            title = {
                                Text(stringResource(Res.string.app_name))
                            },
                            navigationIcon = {
                                IconButton(onClick = { }) {
                                    Icon(
                                        imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                                        contentDescription = "TODO"
                                    )
                                }
                            }
                        )
                    }
                ) { paddingValues ->
                    var showContent by remember { mutableStateOf(false) }
                    Column(
                        Modifier.padding(paddingValues).fillMaxWidth(),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Button(onClick = { showContent = !showContent }) {
                            Text("Click me!")
                        }
                        AnimatedVisibility(showContent) {
                            val greeting = remember { Greeting().greet() }
                            Column(
                                Modifier.fillMaxWidth(),
                                horizontalAlignment = Alignment.CenterHorizontally
                            ) {
                                AsyncImage(
                                    model = "https://wallpapercave.com/wp/wp3724772.jpg",
                                    contentDescription = null,
                                )
                                Text("Compose: $greeting")
                            }
                        }
                    }
                }
            }
        }
    }
}
